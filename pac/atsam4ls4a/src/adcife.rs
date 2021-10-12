#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x08 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x0c - Status Clear Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x10 - Resistive Touch Screen Register"]
    pub rts: crate::Reg<rts::RTS_SPEC>,
    #[doc = "0x14 - Sequencer Configuration Register"]
    pub seqcfg: crate::Reg<seqcfg::SEQCFG_SPEC>,
    _reserved_6_first_dma_word_cdma: [u8; 0x04],
    #[doc = "0x1c - Timing Configuration Register"]
    pub tim: crate::Reg<tim::TIM_SPEC>,
    #[doc = "0x20 - Internal Timer Register"]
    pub itimer: crate::Reg<itimer::ITIMER_SPEC>,
    #[doc = "0x24 - Window Monitor Configuration Register"]
    pub wcfg: crate::Reg<wcfg::WCFG_SPEC>,
    #[doc = "0x28 - Window Monitor Threshold Configuration Register"]
    pub wth: crate::Reg<wth::WTH_SPEC>,
    #[doc = "0x2c - Sequencer Last Converted Value Register"]
    pub lcv: crate::Reg<lcv::LCV_SPEC>,
    #[doc = "0x30 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x34 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x38 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x3c - Calibration Register"]
    pub calib: crate::Reg<calib::CALIB_SPEC>,
    #[doc = "0x40 - Version Register"]
    pub version: crate::Reg<version::VERSION_SPEC>,
    #[doc = "0x44 - Parameter Register"]
    pub parameter: crate::Reg<parameter::PARAMETER_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x18 - Configuration Direct Memory Access Register"]
    #[inline(always)]
    pub fn second_dma_word_cdma_alt(
        &self,
    ) -> &crate::Reg<second_dma_word_cdma_alt::SECOND_DMA_WORD_CDMA_ALT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<second_dma_word_cdma_alt::SECOND_DMA_WORD_CDMA_ALT_SPEC>)
        }
    }
    #[doc = "0x18 - Configuration Direct Memory Access Register"]
    #[inline(always)]
    pub fn first_dma_word_cdma(
        &self,
    ) -> &crate::Reg<first_dma_word_cdma::FIRST_DMA_WORD_CDMA_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<first_dma_word_cdma::FIRST_DMA_WORD_CDMA_SPEC>)
        }
    }
}
#[doc = "CALIB register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "Calibration Register"]
pub mod calib;
#[doc = "FIRST_DMA_WORD_CDMA register accessor: an alias for `Reg<FIRST_DMA_WORD_CDMA_SPEC>`"]
pub type FIRST_DMA_WORD_CDMA = crate::Reg<first_dma_word_cdma::FIRST_DMA_WORD_CDMA_SPEC>;
#[doc = "Configuration Direct Memory Access Register"]
pub mod first_dma_word_cdma;
#[doc = "SECOND_DMA_WORD_CDMA_ALT register accessor: an alias for `Reg<SECOND_DMA_WORD_CDMA_ALT_SPEC>`"]
pub type SECOND_DMA_WORD_CDMA_ALT =
    crate::Reg<second_dma_word_cdma_alt::SECOND_DMA_WORD_CDMA_ALT_SPEC>;
#[doc = "Configuration Direct Memory Access Register"]
pub mod second_dma_word_cdma_alt;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ITIMER register accessor: an alias for `Reg<ITIMER_SPEC>`"]
pub type ITIMER = crate::Reg<itimer::ITIMER_SPEC>;
#[doc = "Internal Timer Register"]
pub mod itimer;
#[doc = "LCV register accessor: an alias for `Reg<LCV_SPEC>`"]
pub type LCV = crate::Reg<lcv::LCV_SPEC>;
#[doc = "Sequencer Last Converted Value Register"]
pub mod lcv;
#[doc = "PARAMETER register accessor: an alias for `Reg<PARAMETER_SPEC>`"]
pub type PARAMETER = crate::Reg<parameter::PARAMETER_SPEC>;
#[doc = "Parameter Register"]
pub mod parameter;
#[doc = "RTS register accessor: an alias for `Reg<RTS_SPEC>`"]
pub type RTS = crate::Reg<rts::RTS_SPEC>;
#[doc = "Resistive Touch Screen Register"]
pub mod rts;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "SEQCFG register accessor: an alias for `Reg<SEQCFG_SPEC>`"]
pub type SEQCFG = crate::Reg<seqcfg::SEQCFG_SPEC>;
#[doc = "Sequencer Configuration Register"]
pub mod seqcfg;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "TIM register accessor: an alias for `Reg<TIM_SPEC>`"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "Timing Configuration Register"]
pub mod tim;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
#[doc = "WCFG register accessor: an alias for `Reg<WCFG_SPEC>`"]
pub type WCFG = crate::Reg<wcfg::WCFG_SPEC>;
#[doc = "Window Monitor Configuration Register"]
pub mod wcfg;
#[doc = "WTH register accessor: an alias for `Reg<WTH_SPEC>`"]
pub type WTH = crate::Reg<wth::WTH_SPEC>;
#[doc = "Window Monitor Threshold Configuration Register"]
pub mod wth;
