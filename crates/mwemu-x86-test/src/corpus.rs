//! Parser for the current pooled x86Tester text corpus format.
//!
//! ```text
//! data:<pool-count>
//! #<hex bytes 0>
//! #<hex bytes 1>
//! ...
//! instr:<address>;#<opcode_hex>;<assembly>;<row_count>;in=<schema>;out=<schema>
//! <input-pool-indexes>|<output-pool-indexes>
//! <input-pool-indexes>|!<exception>
//! ```
//!
//! Schema entries are comma-separated state keys; each maps positionally to a
//! pooled value index. Pooled values are raw hex bytes in memory order.

use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

/// One (key, value-bytes) binding taken from the pool.
pub type Binding = (String, Vec<u8>);

#[derive(Debug, Clone)]
pub enum Outcome {
    State(Vec<Binding>),
    Exception(String),
}

#[derive(Debug, Clone)]
pub struct Row {
    pub inputs: Vec<Binding>,
    pub output: Outcome,
}

#[derive(Debug, Clone)]
pub struct Case {
    pub address: u64,
    pub opcode: Vec<u8>,
    pub asm: String,
    pub mnemonic: String,
    pub rows: Vec<Row>,
}

fn err<T>(line: usize, message: impl Into<String>) -> Result<T, ParseError> {
    Err(ParseError {
        line,
        message: message.into(),
    })
}

fn parse_hex_bytes(hex: &str) -> Option<Vec<u8>> {
    let hex = hex.trim();
    if hex.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(hex.len() / 2);
    let bytes = hex.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let hi = (bytes[i] as char).to_digit(16)?;
        let lo = (bytes[i + 1] as char).to_digit(16)?;
        out.push(((hi << 4) | lo) as u8);
        i += 2;
    }
    Some(out)
}

fn split_schema(field: &str) -> Vec<String> {
    field
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

fn parse_indices(field: &str, line: usize) -> Result<Vec<usize>, ParseError> {
    let mut indices = Vec::new();
    for part in field.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        match part.parse::<usize>() {
            Ok(i) => indices.push(i),
            Err(_) => return err(line, format!("bad pool index {:?}", part)),
        }
    }
    Ok(indices)
}

fn bind(
    schema: &[String],
    indices: &[usize],
    pool: &[Vec<u8>],
    line: usize,
    which: &str,
) -> Result<Vec<Binding>, ParseError> {
    if schema.len() != indices.len() {
        return err(
            line,
            format!(
                "{which} schema has {} keys but row lists {} values",
                schema.len(),
                indices.len()
            ),
        );
    }
    let mut out = Vec::with_capacity(schema.len());
    for (key, &idx) in schema.iter().zip(indices) {
        let value = pool.get(idx).ok_or_else(|| ParseError {
            line,
            message: format!("pool index {idx} out of range"),
        })?;
        out.push((key.clone(), value.clone()));
    }
    Ok(out)
}

struct Header {
    address: u64,
    opcode: Vec<u8>,
    asm: String,
    row_count: usize,
    in_schema: Vec<String>,
    out_schema: Vec<String>,
}

fn parse_header(body: &str, line: usize) -> Result<Header, ParseError> {
    let parts: Vec<&str> = body.split(';').collect();
    if parts.len() < 6 {
        return err(
            line,
            format!("instr header needs 6 fields, got {}", parts.len()),
        );
    }
    // The trailing two fields are always `in=`/`out=` and the field before them
    // is the row count; the assembly text may itself contain `;`, so rejoin the
    // middle span rather than assuming a fixed position.
    let n = parts.len();
    let address = crate::keys::parse_u64(parts[0]).ok_or_else(|| ParseError {
        line,
        message: format!("bad address {:?}", parts[0]),
    })?;
    let opcode_field = parts[1].strip_prefix('#').unwrap_or(parts[1]);
    let opcode = parse_hex_bytes(opcode_field).ok_or_else(|| ParseError {
        line,
        message: format!("bad opcode hex {:?}", parts[1]),
    })?;
    let asm = parts[2..n - 3].join(";");
    let row_count = parts[n - 3]
        .trim()
        .parse::<usize>()
        .map_err(|_| ParseError {
            line,
            message: format!("bad row count {:?}", parts[n - 3]),
        })?;
    let in_field = parts[n - 2].strip_prefix("in=").ok_or_else(|| ParseError {
        line,
        message: "missing in= field".into(),
    })?;
    let out_field = parts[n - 1]
        .strip_prefix("out=")
        .ok_or_else(|| ParseError {
            line,
            message: "missing out= field".into(),
        })?;
    Ok(Header {
        address,
        opcode,
        asm,
        row_count,
        in_schema: split_schema(in_field),
        out_schema: split_schema(out_field),
    })
}

pub fn parse(text: &str) -> Result<Vec<Case>, ParseError> {
    let mut lines = text.lines().enumerate().peekable();

    // Skip blank lines up to the `data:` header.
    let mut pool_count = None;
    while let Some((no, raw)) = lines.peek().copied() {
        let line = raw.trim();
        if line.is_empty() {
            lines.next();
            continue;
        }
        if let Some(rest) = line.strip_prefix("data:") {
            pool_count = Some(rest.trim().parse::<usize>().map_err(|_| ParseError {
                line: no + 1,
                message: format!("bad data count {:?}", rest),
            })?);
            lines.next();
            break;
        }
        return err(no + 1, format!("expected `data:` header, got {:?}", line));
    }
    let pool_count = match pool_count {
        Some(n) => n,
        None => return Ok(Vec::new()),
    };

    // Pool entries: consecutive `#`-prefixed lines.
    let mut pool: Vec<Vec<u8>> = Vec::with_capacity(pool_count);
    while let Some((no, raw)) = lines.peek().copied() {
        let line = raw.trim();
        if let Some(hex) = line.strip_prefix('#') {
            let bytes = parse_hex_bytes(hex).ok_or_else(|| ParseError {
                line: no + 1,
                message: format!("bad pool hex {:?}", hex),
            })?;
            pool.push(bytes);
            lines.next();
        } else {
            break;
        }
    }

    // Instruction blocks.
    let mut cases = Vec::new();
    while let Some((no, raw)) = lines.next() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let body = match line.strip_prefix("instr:") {
            Some(b) => b,
            None => return err(no + 1, format!("expected `instr:` line, got {:?}", line)),
        };
        let header = parse_header(body, no + 1)?;
        let mnemonic = header
            .asm
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();

        let mut rows = Vec::with_capacity(header.row_count);
        for _ in 0..header.row_count {
            let (row_no, row_raw) = loop {
                match lines.next() {
                    Some((rno, rraw)) if !rraw.trim().is_empty() => break (rno, rraw),
                    Some(_) => continue,
                    None => return err(no + 1, "unexpected EOF while reading state rows"),
                }
            };
            let row_line = row_raw.trim();
            let (lhs, rhs) = row_line.split_once('|').ok_or_else(|| ParseError {
                line: row_no + 1,
                message: "state row must contain `|`".into(),
            })?;

            let in_indices = parse_indices(lhs, row_no + 1)?;
            let inputs = bind(&header.in_schema, &in_indices, &pool, row_no + 1, "in")?;

            let output = if let Some(exc) = rhs.trim().strip_prefix('!') {
                Outcome::Exception(exc.trim().to_string())
            } else {
                let out_indices = parse_indices(rhs, row_no + 1)?;
                Outcome::State(bind(
                    &header.out_schema,
                    &out_indices,
                    &pool,
                    row_no + 1,
                    "out",
                )?)
            };

            rows.push(Row { inputs, output });
        }

        cases.push(Case {
            address: header.address,
            opcode: header.opcode,
            asm: header.asm,
            mnemonic,
            rows,
        });
    }

    Ok(cases)
}
