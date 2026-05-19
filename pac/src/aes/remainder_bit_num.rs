#[doc = "Register `REMAINDER_BIT_NUM` reader"]
pub type R = crate::R<RemainderBitNumSpec>;
#[doc = "Register `REMAINDER_BIT_NUM` writer"]
pub type W = crate::W<RemainderBitNumSpec>;
#[doc = "Field `REMAINDER_BIT_NUM` reader - Those bits stores the number of remainder bit."]
pub type RemainderBitNumR = crate::FieldReader;
#[doc = "Field `REMAINDER_BIT_NUM` writer - Those bits stores the number of remainder bit."]
pub type RemainderBitNumW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Those bits stores the number of remainder bit."]
    #[inline(always)]
    pub fn remainder_bit_num(&self) -> RemainderBitNumR {
        RemainderBitNumR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Those bits stores the number of remainder bit."]
    #[inline(always)]
    pub fn remainder_bit_num(&mut self) -> RemainderBitNumW<'_, RemainderBitNumSpec> {
        RemainderBitNumW::new(self, 0)
    }
}
#[doc = "AES remainder bit number register\n\nYou can [`read`](crate::Reg::read) this register and get [`remainder_bit_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remainder_bit_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemainderBitNumSpec;
impl crate::RegisterSpec for RemainderBitNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remainder_bit_num::R`](R) reader structure"]
impl crate::Readable for RemainderBitNumSpec {}
#[doc = "`write(|w| ..)` method takes [`remainder_bit_num::W`](W) writer structure"]
impl crate::Writable for RemainderBitNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAINDER_BIT_NUM to value 0"]
impl crate::Resettable for RemainderBitNumSpec {}
