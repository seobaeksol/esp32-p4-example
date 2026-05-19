#[doc = "Register `VID_PKT_STATUS` reader"]
pub type R = crate::R<VidPktStatusSpec>;
#[doc = "Field `DPI_CMD_W_EMPTY` reader - NA"]
pub type DpiCmdWEmptyR = crate::BitReader;
#[doc = "Field `DPI_CMD_W_FULL` reader - NA"]
pub type DpiCmdWFullR = crate::BitReader;
#[doc = "Field `DPI_PLD_W_EMPTY` reader - NA"]
pub type DpiPldWEmptyR = crate::BitReader;
#[doc = "Field `DPI_PLD_W_FULL` reader - NA"]
pub type DpiPldWFullR = crate::BitReader;
#[doc = "Field `DPI_BUFF_PLD_EMPTY` reader - NA"]
pub type DpiBuffPldEmptyR = crate::BitReader;
#[doc = "Field `DPI_BUFF_PLD_FULL` reader - NA"]
pub type DpiBuffPldFullR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dpi_cmd_w_empty(&self) -> DpiCmdWEmptyR {
        DpiCmdWEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn dpi_cmd_w_full(&self) -> DpiCmdWFullR {
        DpiCmdWFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn dpi_pld_w_empty(&self) -> DpiPldWEmptyR {
        DpiPldWEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn dpi_pld_w_full(&self) -> DpiPldWFullR {
        DpiPldWFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn dpi_buff_pld_empty(&self) -> DpiBuffPldEmptyR {
        DpiBuffPldEmptyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn dpi_buff_pld_full(&self) -> DpiBuffPldFullR {
        DpiBuffPldFullR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_pkt_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidPktStatusSpec;
impl crate::RegisterSpec for VidPktStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_pkt_status::R`](R) reader structure"]
impl crate::Readable for VidPktStatusSpec {}
#[doc = "`reset()` method sets VID_PKT_STATUS to value 0x0001_0005"]
impl crate::Resettable for VidPktStatusSpec {
    const RESET_VALUE: u32 = 0x0001_0005;
}
