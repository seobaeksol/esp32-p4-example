#[doc = "Register `IN_STATE` reader"]
pub type R = crate::R<InStateSpec>;
#[doc = "Field `INLINK_DSCR_ADDR_CH0` reader - reserved"]
pub type InlinkDscrAddrCh0R = crate::FieldReader<u32>;
#[doc = "Field `IN_DSCR_STATE_CH0` reader - reserved"]
pub type InDscrStateCh0R = crate::FieldReader;
#[doc = "Field `IN_STATE_CH0` reader - Represents the address of the lower 18 bits of the next receive descriptor to be processed."]
pub type InStateCh0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - reserved"]
    #[inline(always)]
    pub fn inlink_dscr_addr_ch0(&self) -> InlinkDscrAddrCh0R {
        InlinkDscrAddrCh0R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn in_dscr_state_ch0(&self) -> InDscrStateCh0R {
        InDscrStateCh0R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Represents the address of the lower 18 bits of the next receive descriptor to be processed."]
    #[inline(always)]
    pub fn in_state_ch0(&self) -> InStateCh0R {
        InStateCh0R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "Receive status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InStateSpec;
impl crate::RegisterSpec for InStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_state::R`](R) reader structure"]
impl crate::Readable for InStateSpec {}
#[doc = "`reset()` method sets IN_STATE to value 0"]
impl crate::Resettable for InStateSpec {}
