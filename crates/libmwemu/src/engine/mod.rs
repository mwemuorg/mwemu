// Dispatcher for instruction execution
use crate::emu::Emu;
use iced_x86::{Instruction, Mnemonic};

pub mod aarch64;
pub mod instructions;
pub mod logic;

pub fn emulate_instruction(
    emu: &mut Emu,
    ins: &Instruction,
    instruction_sz: usize,
    rep_step: bool,
) -> bool {
    match ins.mnemonic() {
        Mnemonic::Jmp => instructions::jmp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Call => instructions::call::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Push => instructions::push::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pop => instructions::pop::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pusha => instructions::pusha::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pushad => instructions::pushad::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Popad => instructions::popad::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Popcnt => instructions::popcnt::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Lzcnt => instructions::lzcnt::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pdep => instructions::pdep::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pext => instructions::pext::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Andn => instructions::andn::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Bextr => instructions::bextr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Blsi => instructions::blsi::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Blsr => instructions::blsr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Rorx => instructions::rorx::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pabsb => instructions::pabsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pabsw => instructions::pabsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pabsd => instructions::pabsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psignb => instructions::psignb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psignw => instructions::psignw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psignd => instructions::psignd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Palignr => instructions::palignr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Phaddw => instructions::phaddw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Phaddd => instructions::phaddd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Phsubw => instructions::phsubw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Phsubd => instructions::phsubd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Phaddsw => instructions::phaddsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Phsubsw => instructions::phsubsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmaddubsw => instructions::pmaddubsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmulhrsw => instructions::pmulhrsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovsxbw => instructions::pmovsxbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovsxbd => instructions::pmovsxbd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovsxbq => instructions::pmovsxbq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovsxwd => instructions::pmovsxwd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovsxwq => instructions::pmovsxwq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovsxdq => instructions::pmovsxdq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovzxbw => instructions::pmovzxbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovzxbd => instructions::pmovzxbd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovzxbq => instructions::pmovzxbq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovzxwd => instructions::pmovzxwd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovzxwq => instructions::pmovzxwq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovzxdq => instructions::pmovzxdq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmulld => instructions::pmulld::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pminsd => instructions::pminsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmaxsd => instructions::pmaxsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pminud => instructions::pminud::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmaxud => instructions::pmaxud::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpeqq => instructions::pcmpeqq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Packusdw => instructions::packusdw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ptest => instructions::ptest::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpgtq => instructions::pcmpgtq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Crc32 => instructions::crc32::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Adcx => instructions::adcx::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Adox => instructions::adox::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtph2ps => instructions::vcvtph2ps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtps2ph => instructions::vcvtps2ph::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpdpbusd => instructions::vpdpbusd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpdpbusds => instructions::vpdpbusds::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpdpwssd => instructions::vpdpwssd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpdpwssds => instructions::vpdpwssds::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtdq2pd => instructions::vcvtdq2pd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtps2pd => instructions::vcvtps2pd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtpd2ps => instructions::vcvtpd2ps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtpd2dq => instructions::vcvtpd2dq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvttpd2dq => {
            instructions::vcvttpd2dq::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpermq => instructions::vpermq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpermpd => instructions::vpermpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vperm2f128 => {
            instructions::vperm2f128::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vperm2i128 => {
            instructions::vperm2i128::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpermd => instructions::vpermd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpermps => instructions::vpermps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpermilps => instructions::vpermilps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpermilpd => instructions::vpermilpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsllvd => instructions::vpsllvd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsllvq => instructions::vpsllvq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsrlvd => instructions::vpsrlvd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsrlvq => instructions::vpsrlvq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsravd => instructions::vpsravd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vrcpps => instructions::vrcpps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vrsqrtps => instructions::vrsqrtps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vrcpss => instructions::vrcpss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vrsqrtss => instructions::vrsqrtss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpinsrb => instructions::vpinsrb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpinsrd => instructions::vpinsrd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpinsrq => instructions::vpinsrq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpinsrw => instructions::vpinsrw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpextrq => instructions::vpextrq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovss => instructions::vmovss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovsd => instructions::vmovsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovapd => instructions::vmovapd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovupd => instructions::vmovupd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovddup => instructions::vmovddup::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovsldup => instructions::vmovsldup::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovshdup => instructions::vmovshdup::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpslldq => instructions::vpslldq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsrldq => instructions::vpsrldq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpalignr => instructions::vpalignr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vinsertps => instructions::vinsertps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmpsadbw => instructions::vmpsadbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovhlps => instructions::vmovhlps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovlhps => instructions::vmovlhps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vroundss => instructions::vroundss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vroundsd => instructions::vroundsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovmskps => instructions::vmovmskps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovmskpd => instructions::vmovmskpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vtestps => instructions::vtestps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vtestpd => instructions::vtestpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vxorpd => instructions::vxorpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpcmpgtq => instructions::vpcmpgtq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmuludq => instructions::vpmuludq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmuldq => instructions::vpmuldq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpaddsb => instructions::vpaddsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpaddsw => instructions::vpaddsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpaddusb => instructions::vpaddusb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpaddusw => instructions::vpaddusw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsubsb => instructions::vpsubsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsubsw => instructions::vpsubsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsubusb => instructions::vpsubusb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsubusw => instructions::vpsubusw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vhaddps => instructions::vhaddps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vhsubps => instructions::vhsubps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vhaddpd => instructions::vhaddpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vhsubpd => instructions::vhsubpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vgf2p8mulb => {
            instructions::vgf2p8mulb::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpblendd => instructions::vpblendd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vdpps => instructions::vdpps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vdppd => instructions::vdppd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vgf2p8affineqb => {
            instructions::vgf2p8affineqb::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vgf2p8affineinvqb => {
            instructions::vgf2p8affineinvqb::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpextrb => instructions::pextrb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpextrd => instructions::pextrd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpextrw => instructions::pextrw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vphminposuw => {
            instructions::phminposuw::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpcmpestri => {
            instructions::pcmpestri::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpcmpestrm => {
            instructions::pcmpestrm::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpcmpistri => {
            instructions::pcmpistri::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpcmpistrm => {
            instructions::pcmpistrm::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpmovsxbw => instructions::vpmovsxbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovsxbd => instructions::vpmovsxbd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovsxbq => instructions::vpmovsxbq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovsxwd => instructions::vpmovsxwd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovsxwq => instructions::vpmovsxwq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovsxdq => instructions::vpmovsxdq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovzxbw => instructions::vpmovzxbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovzxbd => instructions::vpmovzxbd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovzxbq => instructions::vpmovzxbq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovzxwd => instructions::vpmovzxwd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovzxwq => instructions::vpmovzxwq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovzxdq => instructions::vpmovzxdq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcmpps => instructions::vcmpps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcmppd => instructions::vcmppd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcmpss => instructions::vcmpss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcmpsd => instructions::vcmpsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtdq2ps => instructions::vcvtdq2ps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtps2dq => instructions::vcvtps2dq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvttps2dq => {
            instructions::vcvttps2dq::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vcvtsd2ss => instructions::vcvtsd2ss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtss2sd => instructions::vcvtss2sd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtsi2sd => instructions::vcvtsi2sd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvtsi2ss => instructions::vcvtsi2ss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vextractf128 => {
            instructions::vextractf128::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vextracti128 => {
            instructions::vextracti128::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vinsertf128 => {
            instructions::vinsertf128::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vinserti128 => {
            instructions::vinserti128::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vextractps => {
            instructions::vextractps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vcvtsd2si => instructions::cvtsd2si::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvttsd2si => {
            instructions::cvttsd2si::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vcvtss2si => instructions::cvtss2si::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcvttss2si => {
            instructions::cvttss2si::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd132ps => {
            instructions::vfmadd132ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd132pd => {
            instructions::vfmadd132pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd132ss => {
            instructions::vfmadd132ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd132sd => {
            instructions::vfmadd132sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd213ps => {
            instructions::vfmadd213ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd213pd => {
            instructions::vfmadd213pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd213ss => {
            instructions::vfmadd213ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd213sd => {
            instructions::vfmadd213sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd231ps => {
            instructions::vfmadd231ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd231pd => {
            instructions::vfmadd231pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd231ss => {
            instructions::vfmadd231ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmadd231sd => {
            instructions::vfmadd231sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub132ps => {
            instructions::vfmsub132ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub132pd => {
            instructions::vfmsub132pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub132ss => {
            instructions::vfmsub132ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub132sd => {
            instructions::vfmsub132sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub213ps => {
            instructions::vfmsub213ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub213pd => {
            instructions::vfmsub213pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub213ss => {
            instructions::vfmsub213ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub213sd => {
            instructions::vfmsub213sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub231ps => {
            instructions::vfmsub231ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub231pd => {
            instructions::vfmsub231pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub231ss => {
            instructions::vfmsub231ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsub231sd => {
            instructions::vfmsub231sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd132ps => {
            instructions::vfnmadd132ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd132pd => {
            instructions::vfnmadd132pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd132ss => {
            instructions::vfnmadd132ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd132sd => {
            instructions::vfnmadd132sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd213ps => {
            instructions::vfnmadd213ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd213pd => {
            instructions::vfnmadd213pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd213ss => {
            instructions::vfnmadd213ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd213sd => {
            instructions::vfnmadd213sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd231ps => {
            instructions::vfnmadd231ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd231pd => {
            instructions::vfnmadd231pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd231ss => {
            instructions::vfnmadd231ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmadd231sd => {
            instructions::vfnmadd231sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub132ps => {
            instructions::vfnmsub132ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub132pd => {
            instructions::vfnmsub132pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub132ss => {
            instructions::vfnmsub132ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub132sd => {
            instructions::vfnmsub132sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub213ps => {
            instructions::vfnmsub213ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub213pd => {
            instructions::vfnmsub213pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub213ss => {
            instructions::vfnmsub213ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub213sd => {
            instructions::vfnmsub213sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub231ps => {
            instructions::vfnmsub231ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub231pd => {
            instructions::vfnmsub231pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub231ss => {
            instructions::vfnmsub231ss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfnmsub231sd => {
            instructions::vfnmsub231sd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmaddsub132ps => {
            instructions::vfmaddsub132ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmaddsub132pd => {
            instructions::vfmaddsub132pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmaddsub213ps => {
            instructions::vfmaddsub213ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmaddsub213pd => {
            instructions::vfmaddsub213pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmaddsub231ps => {
            instructions::vfmaddsub231ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmaddsub231pd => {
            instructions::vfmaddsub231pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsubadd132ps => {
            instructions::vfmsubadd132ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsubadd132pd => {
            instructions::vfmsubadd132pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsubadd213ps => {
            instructions::vfmsubadd213ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsubadd213pd => {
            instructions::vfmsubadd213pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsubadd231ps => {
            instructions::vfmsubadd231ps::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vfmsubadd231pd => {
            instructions::vfmsubadd231pd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpbroadcastw => {
            instructions::vpbroadcastw::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpbroadcastd => {
            instructions::vpbroadcastd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpbroadcastq => {
            instructions::vpbroadcastq::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vbroadcastss => {
            instructions::vbroadcastss::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vbroadcastsd => {
            instructions::vbroadcastsd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpshufd => instructions::vpshufd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpshuflw => instructions::vpshuflw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpshufhw => instructions::vpshufhw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vroundps => instructions::vroundps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vroundpd => instructions::vroundpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vshufps => instructions::vshufps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vshufpd => instructions::vshufpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpblendw => instructions::vpblendw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vblendps => instructions::vblendps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vblendpd => instructions::vblendpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vphaddw => instructions::vphaddw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vphsubw => instructions::vphsubw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vphaddd => instructions::vphaddd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vphsubd => instructions::vphsubd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vphaddsw => instructions::vphaddsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vphsubsw => instructions::vphsubsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsignb => instructions::vpsignb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsignw => instructions::vpsignw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsignd => instructions::vpsignd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmaddwd => instructions::vpmaddwd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmaddubsw => {
            instructions::vpmaddubsw::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpmulhrsw => instructions::vpmulhrsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsadbw => instructions::vpsadbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vptest => instructions::vptest::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsllw => instructions::vpsllw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpslld => instructions::vpslld::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsllq => instructions::vpsllq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsrlw => instructions::vpsrlw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsrld => instructions::vpsrld::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsrlq => instructions::vpsrlq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsraw => instructions::vpsraw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsrad => instructions::vpsrad::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpunpcklbw => {
            instructions::vpunpcklbw::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpunpckhbw => {
            instructions::vpunpckhbw::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpunpcklwd => {
            instructions::vpunpcklwd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpunpckhwd => {
            instructions::vpunpckhwd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpunpckldq => {
            instructions::vpunpckldq::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpunpckhdq => {
            instructions::vpunpckhdq::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpunpcklqdq => {
            instructions::vpunpcklqdq::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpunpckhqdq => {
            instructions::vpunpckhqdq::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpshufb => instructions::vpshufb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpackusdw => instructions::vpackusdw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpacksswb => instructions::vpacksswb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vaesenc => instructions::vaesenc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vaesenclast => {
            instructions::vaesenclast::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vaesdec => instructions::vaesdec::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vaesdeclast => {
            instructions::vaesdeclast::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vaesimc => instructions::vaesimc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vaeskeygenassist => {
            instructions::vaeskeygenassist::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpclmulqdq => {
            instructions::vpclmulqdq::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vcomiss => instructions::vcomiss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vcomisd => instructions::vcomisd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vucomiss => instructions::vucomiss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vucomisd => instructions::vucomisd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vaddss => instructions::vaddss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vaddsd => instructions::vaddsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vsubss => instructions::vsubss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vsubsd => instructions::vsubsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmulss => instructions::vmulss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmulsd => instructions::vmulsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vdivss => instructions::vdivss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vdivsd => instructions::vdivsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmaxss => instructions::vmaxss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmaxsd => instructions::vmaxsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vminss => instructions::vminss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vminsd => instructions::vminsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vsqrtss => instructions::vsqrtss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vsqrtsd => instructions::vsqrtsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vaddsubps => instructions::vaddsubps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vaddsubpd => instructions::vaddsubpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vaddps => instructions::vaddps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vaddpd => instructions::vaddpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vsubps => instructions::vsubps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vsubpd => instructions::vsubpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmulps => instructions::vmulps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmulpd => instructions::vmulpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vdivps => instructions::vdivps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vdivpd => instructions::vdivpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmaxps => instructions::vmaxps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmaxpd => instructions::vmaxpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vminps => instructions::vminps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vminpd => instructions::vminpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vunpcklps => instructions::vunpcklps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vunpckhps => instructions::vunpckhps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vunpcklpd => instructions::vunpcklpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vunpckhpd => instructions::vunpckhpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpackuswb => instructions::vpackuswb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpackssdw => instructions::vpackssdw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vsqrtps => instructions::vsqrtps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vsqrtpd => instructions::vsqrtpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpabsb => instructions::vpabsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpabsw => instructions::vpabsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpabsd => instructions::vpabsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpand => instructions::vpand::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vandps => instructions::vandps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vandpd => instructions::vandpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vandnps => instructions::vandnps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vandnpd => instructions::vandnpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vorps => instructions::vorps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vorpd => instructions::vorpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpaddw => instructions::vpaddw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpaddd => instructions::vpaddd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpaddq => instructions::vpaddq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsubw => instructions::vpsubw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsubd => instructions::vpsubd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsubq => instructions::vpsubq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmullw => instructions::vpmullw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmulld => instructions::vpmulld::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmulhw => instructions::vpmulhw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmulhuw => instructions::vpmulhuw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpcmpeqw => instructions::vpcmpeqw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpcmpeqd => instructions::vpcmpeqd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpcmpeqq => instructions::vpcmpeqq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpcmpgtw => instructions::vpcmpgtw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpcmpgtd => instructions::vpcmpgtd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpavgb => instructions::vpavgb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpavgw => instructions::vpavgw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmaxub => instructions::vpmaxub::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmaxuw => instructions::vpmaxuw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmaxud => instructions::vpmaxud::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpminuw => instructions::vpminuw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpminud => instructions::vpminud::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmaxsb => instructions::vpmaxsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmaxsw => instructions::vpmaxsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmaxsd => instructions::vpmaxsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpminsb => instructions::vpminsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpminsw => instructions::vpminsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpminsd => instructions::vpminsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpestri => instructions::pcmpestri::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpestrm => instructions::pcmpestrm::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sha1msg1 => instructions::sha1msg1::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sha1msg2 => instructions::sha1msg2::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sha1nexte => instructions::sha1nexte::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sha1rnds4 => instructions::sha1rnds4::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sha256msg1 => {
            instructions::sha256msg1::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Sha256msg2 => {
            instructions::sha256msg2::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Sha256rnds2 => {
            instructions::sha256rnds2::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Aesenc => instructions::aesenc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Aesenclast => {
            instructions::aesenclast::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Aesdec => instructions::aesdec::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Aesdeclast => {
            instructions::aesdeclast::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Aesimc => instructions::aesimc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Aeskeygenassist => {
            instructions::aeskeygenassist::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Pclmulqdq => instructions::pclmulqdq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Gf2p8mulb => instructions::gf2p8mulb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Gf2p8affineqb => {
            instructions::gf2p8affineqb::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Gf2p8affineinvqb => {
            instructions::gf2p8affineinvqb::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Dpps => instructions::dpps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Dppd => instructions::dppd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Mpsadbw => instructions::mpsadbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Phminposuw => {
            instructions::phminposuw::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Rcpps => instructions::rcpps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Rcpss => instructions::rcpss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Rsqrtps => instructions::rsqrtps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Rsqrtss => instructions::rsqrtss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Roundps => instructions::roundps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Roundpd => instructions::roundpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Roundss => instructions::roundss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Roundsd => instructions::roundsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Blendps => instructions::blendps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Blendpd => instructions::blendpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Blendvps => instructions::blendvps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Blendvpd => instructions::blendvpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pblendw => instructions::pblendw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pblendvb => instructions::pblendvb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pextrb => instructions::pextrb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pextrd => instructions::pextrd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pinsrb => instructions::pinsrb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pinsrd => instructions::pinsrd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pinsrq => instructions::pinsrq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Extractps => instructions::extractps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Insertps => instructions::insertps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psadbw => instructions::psadbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmaxsb => instructions::pmaxsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmaxsw => instructions::pmaxsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmaxub => instructions::pmaxub::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmaxuw => instructions::pmaxuw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pminsb => instructions::pminsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pminsw => instructions::pminsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pminuw => instructions::pminuw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pavgb => instructions::pavgb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pavgw => instructions::pavgw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psubusw => instructions::psubusw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmulhuw => instructions::pmulhuw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmuludq => instructions::pmuludq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmuldq => instructions::pmuldq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Punpckhqdq => {
            instructions::punpckhqdq::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Andnps => instructions::andnps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Andnpd => instructions::andnpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Unpcklps => instructions::unpcklps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Unpckhps => instructions::unpckhps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Unpcklpd => instructions::unpcklpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Unpckhpd => instructions::unpckhpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Shufps => instructions::shufps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movddup => instructions::movddup::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movsldup => instructions::movsldup::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movshdup => instructions::movshdup::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movupd => instructions::movupd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movmskps => instructions::movmskps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movmskpd => instructions::movmskpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sahf => instructions::sahf::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Divps => instructions::divps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Divpd => instructions::divpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Divss => instructions::divss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Divsd => instructions::divsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Maxps => instructions::maxps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Maxpd => instructions::maxpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Maxss => instructions::maxss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Maxsd => instructions::maxsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Minps => instructions::minps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Minpd => instructions::minpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Minss => instructions::minss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Minsd => instructions::minsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sqrtps => instructions::sqrtps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sqrtpd => instructions::sqrtpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sqrtss => instructions::sqrtss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sqrtsd => instructions::sqrtsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Addsubps => instructions::addsubps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Addsubpd => instructions::addsubpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Haddps => instructions::haddps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Haddpd => instructions::haddpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Hsubps => instructions::hsubps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Hsubpd => instructions::hsubpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmpps => instructions::cmpps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmppd => instructions::cmppd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmpss => instructions::cmpss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtdq2ps => instructions::cvtdq2ps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtdq2pd => instructions::cvtdq2pd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtps2pd => instructions::cvtps2pd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtpd2ps => instructions::cvtpd2ps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtsd2ss => instructions::cvtsd2ss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtss2sd => instructions::cvtss2sd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtps2dq => instructions::cvtps2dq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvttps2dq => instructions::cvttps2dq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtpd2dq => instructions::cvtpd2dq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvttpd2dq => instructions::cvttpd2dq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtss2si => instructions::cvtss2si::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvttss2si => instructions::cvttss2si::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cdqe => instructions::cdqe::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cdq => instructions::cdq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cqo => instructions::cqo::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ret => instructions::ret::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Xchg => instructions::xchg::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Aaa => instructions::aaa::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Aas => instructions::aas::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Aam => instructions::aam::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Aad => instructions::aad::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Les => instructions::les::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Mov => instructions::mov::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movnti => instructions::movnti::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Xor => instructions::xor::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Add => instructions::add::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Adc => instructions::adc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sbb => instructions::sbb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sub => instructions::sub::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Inc => instructions::inc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Dec => instructions::dec::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Neg => instructions::neg::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Not => instructions::not::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::And => instructions::and::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Or => instructions::or::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sal => instructions::sal::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sar => instructions::sar::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sarx => instructions::sarx::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Shlx => instructions::shlx::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Shrx => instructions::shrx::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Shl => instructions::shl::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Shr => instructions::shr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ror => instructions::ror::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Rcr => instructions::rcr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Rol => instructions::rol::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Rcl => instructions::rcl::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Mul => instructions::mul::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Mulx => instructions::mulx::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Div => instructions::div::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Idiv => instructions::idiv::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Imul => instructions::imul::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Bt => instructions::bt::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Btc => instructions::btc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Bts => instructions::bts::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Btr => instructions::btr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Bsf => instructions::bsf::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Bsr => instructions::bsr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Blsmsk => instructions::blsmsk::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Bzhi => instructions::bzhi::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Bswap => instructions::bswap::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Xadd => instructions::xadd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ucomiss => instructions::ucomiss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ucomisd => instructions::ucomisd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Comisd => instructions::comisd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Comiss => instructions::comiss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movss => instructions::movss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movsxd => instructions::movsxd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movsx => instructions::movsx::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movzx => instructions::movzx::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movsb => instructions::movsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movsw => instructions::movsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movsq => instructions::movsq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movsd => instructions::movsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmova => instructions::cmova::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovae => instructions::cmovae::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovb => instructions::cmovb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovbe => instructions::cmovbe::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmove => instructions::cmove::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovg => instructions::cmovg::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovge => instructions::cmovge::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovl => instructions::cmovl::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovle => instructions::cmovle::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovno => instructions::cmovno::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovne => instructions::cmovne::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovp => instructions::cmovp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovnp => instructions::cmovnp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovs => instructions::cmovs::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovns => instructions::cmovns::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmovo => instructions::cmovo::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Seta => instructions::seta::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setae => instructions::setae::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setb => instructions::setb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setbe => instructions::setbe::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sete => instructions::sete::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setg => instructions::setg::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setge => instructions::setge::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setl => instructions::setl::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setle => instructions::setle::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setne => instructions::setne::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setno => instructions::setno::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setnp => instructions::setnp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setns => instructions::setns::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Seto => instructions::seto::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Setp => instructions::setp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sets => instructions::sets::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Stosb => instructions::stosb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Stosw => instructions::stosw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Stosd => instructions::stosd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Stosq => instructions::stosq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Scasb => instructions::scasb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Scasw => instructions::scasw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Scasd => instructions::scasd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Scasq => instructions::scasq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Test => instructions::test::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmpxchg => instructions::cmpxchg::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmpxchg8b => instructions::cmpxchg8b::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmpxchg16b => {
            instructions::cmpxchg16b::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Cmp => instructions::cmp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmpsq => instructions::cmpsq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmpsd => instructions::cmpsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmpsw => instructions::cmpsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmpsb => instructions::cmpsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jo => instructions::jo::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jno => instructions::jno::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Js => instructions::js::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jns => instructions::jns::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Je => instructions::je::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jne => instructions::jne::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jb => instructions::jb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jae => instructions::jae::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jbe => instructions::jbe::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ja => instructions::ja::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jl => instructions::jl::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jge => instructions::jge::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jle => instructions::jle::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jg => instructions::jg::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jp => instructions::jp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jnp => instructions::jnp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jcxz => instructions::jcxz::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jecxz => instructions::jecxz::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Jrcxz => instructions::jrcxz::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Int3 => instructions::int3::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Nop => instructions::nop::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fnop => instructions::fnop::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Mfence => instructions::mfence::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Lfence => instructions::lfence::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sfence => instructions::sfence::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cpuid => instructions::cpuid::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Clc => instructions::clc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Rdtsc => instructions::rdtsc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Rdtscp => instructions::rdtscp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Loop => instructions::r#loop::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Loope => instructions::loope::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Loopne => instructions::loopne::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Lea => instructions::lea::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Leave => instructions::leave::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Int => instructions::int::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Syscall => instructions::syscall::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Std => instructions::std::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Stc => instructions::stc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cmc => instructions::cmc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cld => instructions::cld::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Lodsq => instructions::lodsq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Lodsd => instructions::lodsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Lodsw => instructions::lodsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Lodsb => instructions::lodsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cbw => instructions::cbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cwde => instructions::cwde::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cwd => instructions::cwd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fninit => instructions::fninit::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Finit => instructions::finit::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ffree => instructions::ffree::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fbld => instructions::fbld::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fldcw => instructions::fldcw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fnstenv => instructions::fnstenv::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fld => instructions::fld::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fldz => instructions::fldz::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fld1 => instructions::fld1::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fldpi => instructions::fldpi::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fldl2t => instructions::fldl2t::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fldlg2 => instructions::fldlg2::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fldln2 => instructions::fldln2::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fldl2e => instructions::fldl2e::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fst => instructions::fst::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fsubrp => instructions::fsubrp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fstp => instructions::fstp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fincstp => instructions::fincstp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fild => instructions::fild::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fist => instructions::fist::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fxtract => instructions::fxtract::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fxsave => instructions::fxsave::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fxrstor => instructions::fxrstor::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fistp => instructions::fistp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcmove => instructions::fcmove::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcmovb => instructions::fcmovb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcmovbe => instructions::fcmovbe::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcmovu => instructions::fcmovu::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcmovnb => instructions::fcmovnb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcmovne => instructions::fcmovne::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcmovnbe => instructions::fcmovnbe::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcmovnu => instructions::fcmovnu::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fxch => instructions::fxch::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fsqrt => instructions::fsqrt::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fchs => instructions::fchs::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fptan => instructions::fptan::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fmulp => instructions::fmulp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fdivp => instructions::fdivp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fsubp => instructions::fsubp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fsubr => instructions::fsubr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fsub => instructions::fsub::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fadd => instructions::fadd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fucom => instructions::fucom::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::F2xm1 => instructions::f2xm1::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fyl2x => instructions::fyl2x::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fyl2xp1 => instructions::fyl2xp1::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Faddp => instructions::faddp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fnclex => instructions::fnclex::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcom => instructions::fcom::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fmul => instructions::fmul::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fabs => instructions::fabs::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fsin => instructions::fsin::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcos => instructions::fcos::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fdiv => instructions::fdiv::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fdivr => instructions::fdivr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fdivrp => instructions::fdivrp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fpatan => instructions::fpatan::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fprem => instructions::fprem::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fprem1 => instructions::fprem1::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Popf => instructions::popf::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Popfd => instructions::popfd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Popfq => instructions::popfq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Daa => instructions::daa::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Shld => instructions::shld::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Shrd => instructions::shrd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Sysenter => instructions::sysenter::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpeqd => instructions::pcmpeqd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psubusb => instructions::psubusb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Punpckhbw => instructions::punpckhbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pand => instructions::pand::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Por => instructions::por::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pxor => instructions::pxor::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Punpcklbw => instructions::punpcklbw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Punpcklwd => instructions::punpcklwd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Xorps => instructions::xorps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Xorpd => instructions::xorpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psubb => instructions::psubb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psubw => instructions::psubw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psubd => instructions::psubd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psubq => instructions::psubq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movhpd => instructions::movhpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movlpd => instructions::movlpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movlps => instructions::movlps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtsi2sd => instructions::cvtsi2sd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvttsd2si => instructions::cvttsd2si::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtsd2si => instructions::cvtsd2si::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Cvtsi2ss => instructions::cvtsi2ss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movhps => instructions::movhps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Punpcklqdq => {
            instructions::punpcklqdq::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Movq => instructions::movq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Punpckhdq => instructions::punpckhdq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Punpckldq => instructions::punpckldq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movd => instructions::movd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movbe => instructions::movbe::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movdqa => instructions::movdqa::execute(emu, ins, instruction_sz, rep_step),
        // `andps` and `andpd` are both a full 128-bit bitwise AND; the ps/pd
        // element width is irrelevant to the operation, so they share a handler.
        Mnemonic::Andpd | Mnemonic::Andps => {
            instructions::andpd::execute(emu, ins, instruction_sz, rep_step)
        }
        // `orps`/`orpd`: identical full 128-bit bitwise OR, share a handler.
        Mnemonic::Orpd | Mnemonic::Orps => {
            instructions::orpd::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Pextrw => instructions::pextrw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pinsrw => instructions::pinsrw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Addps => instructions::addps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Addpd => instructions::addpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Addsd => instructions::addsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Addss => instructions::addss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Subps => instructions::subps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Subpd => instructions::subpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Subsd => instructions::subsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Subss => instructions::subss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Mulpd => instructions::mulpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Mulps => instructions::mulps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Mulsd => instructions::mulsd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Mulss => instructions::mulss::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Packsswb => instructions::packsswb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Packssdw => instructions::packssdw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psrldq => instructions::psrldq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pslld => instructions::pslld::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pslldq => instructions::pslldq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psllq => instructions::psllq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psllw => instructions::psllw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Paddsw => instructions::paddsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Paddsb => instructions::paddsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psrad => instructions::psrad::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Paddusb => instructions::paddusb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Paddb => instructions::paddb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Paddusw => instructions::paddusw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Paddw => instructions::paddw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pshufd => instructions::pshufd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Shufpd => instructions::shufpd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movups => instructions::movups::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movdqu => instructions::movdqu::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vzeroupper => {
            instructions::vzeroupper::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vmovups => instructions::vmovups::execute(emu, ins, instruction_sz, rep_step),
        // VMOVAPS is VMOVUPS minus the alignment fault, which we don't model.
        Mnemonic::Vmovaps => instructions::vmovups::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovdqu => instructions::vmovdqu::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovdqa => instructions::vmovdqa::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movaps => instructions::movaps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movapd => instructions::movapd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovd => instructions::vmovd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vmovq => instructions::vmovq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpbroadcastb => {
            instructions::vpbroadcastb::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Vpandn => instructions::vpandn::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpor => instructions::vpor::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpaddb => instructions::vpaddb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpcmpgtb => instructions::vpcmpgtb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpsubb => instructions::vpsubb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpxor => instructions::vpxor::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vxorps => instructions::vxorps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpeqb => instructions::pcmpeqb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psubsb => instructions::psubsb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fcomp => instructions::fcomp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psrlq => instructions::psrlq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psubsw => instructions::psubsw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fsincos => instructions::fsincos::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Packuswb => instructions::packuswb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pandn => instructions::pandn::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psrld => instructions::psrld::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Punpckhwd => instructions::punpckhwd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psraw => instructions::psraw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Frndint => instructions::frndint::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Psrlw => instructions::psrlw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Paddd => instructions::paddd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Paddq => instructions::paddq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fscale => instructions::fscale::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpcmpeqb => instructions::vpcmpeqb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmullw => instructions::pmullw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmulhw => instructions::pmulhw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmovmskb => instructions::pmovmskb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpmovmskb => instructions::vpmovmskb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Vpminub => instructions::vpminub::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pminub => instructions::pminub::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpistri => instructions::pcmpistri::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpistrm => instructions::pcmpistrm::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pshufb => instructions::pshufb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fdecstp => instructions::fdecstp::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ftst => instructions::ftst::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Emms => instructions::emms::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fxam => instructions::fxam::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpgtw => instructions::pcmpgtw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpgtb => instructions::pcmpgtb::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpeqw => instructions::pcmpeqw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pcmpgtd => instructions::pcmpgtd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pmaddwd => instructions::pmaddwd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Tzcnt => instructions::tzcnt::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Xgetbv => instructions::xgetbv::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Arpl => instructions::arpl::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pushf => instructions::pushf::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pushfd => instructions::pushfd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pushfq => instructions::pushfq::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Bound => instructions::bound::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Lahf => instructions::lahf::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Salc => instructions::salc::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movlhps => instructions::movlhps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Movhlps => instructions::movhlps::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pshuflw => instructions::pshuflw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pshufhw => instructions::pshufhw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Stmxcsr => instructions::stmxcsr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ldmxcsr => instructions::ldmxcsr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Fnstcw => instructions::fnstcw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Prefetchnta => {
            instructions::prefetchnta::execute(emu, ins, instruction_sz, rep_step)
        }
        Mnemonic::Prefetchw => instructions::prefetchw::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Pause => instructions::pause::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Wait => instructions::wait::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Mwait => instructions::mwait::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Endbr64 => instructions::endbr64::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Endbr32 => instructions::endbr32::execute(emu, ins, instruction_sz, rep_step),
        // CET shadow-stack instructions — NOPs on hardware without CET enabled.
        // ntdll uses RDSSPQ in exception-cleanup paths and expects the destination
        // to keep its prior value (real CPUs without CET also treat these as NOPs).
        Mnemonic::Rdsspd
        | Mnemonic::Rdsspq
        | Mnemonic::Incsspd
        | Mnemonic::Incsspq
        | Mnemonic::Rstorssp
        | Mnemonic::Saveprevssp
        | Mnemonic::Setssbsy
        | Mnemonic::Clrssbsy
        | Mnemonic::Wrssd
        | Mnemonic::Wrssq
        | Mnemonic::Wrussd
        | Mnemonic::Wrussq => instructions::cet::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Enqcmd => instructions::enqcmd::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Enqcmds => instructions::enqcmds::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Enter => instructions::enter::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Rdmsr => instructions::rdmsr::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ud0 => instructions::ud::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ud1 => instructions::ud::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Ud2 => instructions::ud::execute(emu, ins, instruction_sz, rep_step),
        Mnemonic::Hlt => instructions::hlt::execute(emu, ins, instruction_sz, rep_step),
        _ => {
            log::trace!(
                "{} Unimplemented instruction: {:?}",
                emu.pos,
                ins.mnemonic()
            );
            false
        }
    }
}
