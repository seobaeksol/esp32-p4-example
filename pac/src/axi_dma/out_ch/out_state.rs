#[doc = "Register `OUT_STATE` reader"]
pub type R = crate::R<OutStateSpec>;
#[doc = "Field `OUTLINK_DSCR_ADDR` reader - This register stores the current outlink descriptor's address."]
pub type OutlinkDscrAddrR = crate::FieldReader<u32>;
#[doc = "Field `OUT_DSCR_STATE` reader - reserved"]
pub type OutDscrStateR = crate::FieldReader;
#[doc = "Field `OUT_STATE` reader - reserved"]
pub type OutStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - This register stores the current outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_dscr_addr(&self) -> OutlinkDscrAddrR {
        OutlinkDscrAddrR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn out_dscr_state(&self) -> OutDscrStateR {
        OutDscrStateR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - reserved"]
    #[inline(always)]
    pub fn out_state(&self) -> OutStateR {
        OutStateR::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "Transmit status of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutStateSpec;
impl crate::RegisterSpec for OutStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_state::R`](R) reader structure"]
impl crate::Readable for OutStateSpec {}
#[doc = "`reset()` method sets OUT_STATE to value 0"]
impl crate::Resettable for OutStateSpec {}
