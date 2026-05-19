#[doc = "Register `CDETECT` reader"]
pub type R = crate::R<CdetectSpec>;
#[doc = "Field `CARD_DETECT_N` reader - Value on sdhost_card_detect_n input ports (1 bit per card), read-only bits. 0 represents presence of card. Only NUM_CARDS number of bits are implemented."]
pub type CardDetectNR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Value on sdhost_card_detect_n input ports (1 bit per card), read-only bits. 0 represents presence of card. Only NUM_CARDS number of bits are implemented."]
    #[inline(always)]
    pub fn card_detect_n(&self) -> CardDetectNR {
        CardDetectNR::new((self.bits & 3) as u8)
    }
}
#[doc = "Card detect register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdetect::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdetectSpec;
impl crate::RegisterSpec for CdetectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdetect::R`](R) reader structure"]
impl crate::Readable for CdetectSpec {}
#[doc = "`reset()` method sets CDETECT to value 0"]
impl crate::Resettable for CdetectSpec {}
