#[doc = "Register `OUT_STATE` reader"]
pub type R = crate::R<OutStateSpec>;
#[doc = "Field `OUTLINK_DSCR_ADDR_CH0` reader - Represents the lower 18 bits of the address of the next transmit descriptor to be processed."]
pub type OutlinkDscrAddrCh0R = crate::FieldReader<u32>;
#[doc = "Field `OUT_DSCR_STATE_CH0` reader - reserved"]
pub type OutDscrStateCh0R = crate::FieldReader;
#[doc = "Field `OUT_STATE_CH0` reader - reserved"]
pub type OutStateCh0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - Represents the lower 18 bits of the address of the next transmit descriptor to be processed."]
    #[inline(always)]
    pub fn outlink_dscr_addr_ch0(&self) -> OutlinkDscrAddrCh0R {
        OutlinkDscrAddrCh0R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn out_dscr_state_ch0(&self) -> OutDscrStateCh0R {
        OutDscrStateCh0R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - reserved"]
    #[inline(always)]
    pub fn out_state_ch0(&self) -> OutStateCh0R {
        OutStateCh0R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "Transmit status of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutStateSpec;
impl crate::RegisterSpec for OutStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_state::R`](R) reader structure"]
impl crate::Readable for OutStateSpec {}
#[doc = "`reset()` method sets OUT_STATE to value 0"]
impl crate::Resettable for OutStateSpec {}
