#[doc = "Register `WRTPRT` reader"]
pub type R = crate::R<WrtprtSpec>;
#[doc = "Field `WRITE_PROTECT` reader - Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented."]
pub type WriteProtectR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented."]
    #[inline(always)]
    pub fn write_protect(&self) -> WriteProtectR {
        WriteProtectR::new((self.bits & 3) as u8)
    }
}
#[doc = "Card write protection (WP) status register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrtprt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrtprtSpec;
impl crate::RegisterSpec for WrtprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrtprt::R`](R) reader structure"]
impl crate::Readable for WrtprtSpec {}
#[doc = "`reset()` method sets WRTPRT to value 0"]
impl crate::Resettable for WrtprtSpec {}
