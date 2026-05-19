#[doc = "Register `BS_THRESHOLD` reader"]
pub type R = crate::R<BsThresholdSpec>;
#[doc = "Register `BS_THRESHOLD` writer"]
pub type W = crate::W<BsThresholdSpec>;
#[doc = "Field `BS_BUFFER_THRESHOLD` reader - Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb."]
pub type BsBufferThresholdR = crate::FieldReader;
#[doc = "Field `BS_BUFFER_THRESHOLD` writer - Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb."]
pub type BsBufferThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb."]
    #[inline(always)]
    pub fn bs_buffer_threshold(&self) -> BsBufferThresholdR {
        BsBufferThresholdR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb."]
    #[inline(always)]
    pub fn bs_buffer_threshold(&mut self) -> BsBufferThresholdW<'_, BsThresholdSpec> {
        BsBufferThresholdW::new(self, 0)
    }
}
#[doc = "Bitstream buffer overflow threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsThresholdSpec;
impl crate::RegisterSpec for BsThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_threshold::R`](R) reader structure"]
impl crate::Readable for BsThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`bs_threshold::W`](W) writer structure"]
impl crate::Writable for BsThresholdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_THRESHOLD to value 0x30"]
impl crate::Resettable for BsThresholdSpec {
    const RESET_VALUE: u32 = 0x30;
}
