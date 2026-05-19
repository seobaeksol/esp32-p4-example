#[doc = "Register `INT_MSK_PKT_FATAL` reader"]
pub type R = crate::R<IntMskPktFatalSpec>;
#[doc = "Register `INT_MSK_PKT_FATAL` writer"]
pub type W = crate::W<IntMskPktFatalSpec>;
#[doc = "Field `MASK_ERR_ECC_DOUBLE` reader - NA"]
pub type MaskErrEccDoubleR = crate::BitReader;
#[doc = "Field `MASK_ERR_ECC_DOUBLE` writer - NA"]
pub type MaskErrEccDoubleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_SHORTER_PAYLOAD` reader - NA"]
pub type MaskShorterPayloadR = crate::BitReader;
#[doc = "Field `MASK_SHORTER_PAYLOAD` writer - NA"]
pub type MaskShorterPayloadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn mask_err_ecc_double(&self) -> MaskErrEccDoubleR {
        MaskErrEccDoubleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn mask_shorter_payload(&self) -> MaskShorterPayloadR {
        MaskShorterPayloadR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn mask_err_ecc_double(&mut self) -> MaskErrEccDoubleW<'_, IntMskPktFatalSpec> {
        MaskErrEccDoubleW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn mask_shorter_payload(&mut self) -> MaskShorterPayloadW<'_, IntMskPktFatalSpec> {
        MaskShorterPayloadW::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk_pkt_fatal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk_pkt_fatal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMskPktFatalSpec;
impl crate::RegisterSpec for IntMskPktFatalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_msk_pkt_fatal::R`](R) reader structure"]
impl crate::Readable for IntMskPktFatalSpec {}
#[doc = "`write(|w| ..)` method takes [`int_msk_pkt_fatal::W`](W) writer structure"]
impl crate::Writable for IntMskPktFatalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_MSK_PKT_FATAL to value 0"]
impl crate::Resettable for IntMskPktFatalSpec {}
