#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `INT_ENA` reader - Write 1 to enable the RSA interrupt."]
pub type IntEnaR = crate::BitReader;
#[doc = "Field `INT_ENA` writer - Write 1 to enable the RSA interrupt."]
pub type IntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable the RSA interrupt."]
    #[inline(always)]
    pub fn int_ena(&self) -> IntEnaR {
        IntEnaR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable the RSA interrupt."]
    #[inline(always)]
    pub fn int_ena(&mut self) -> IntEnaW<'_, IntEnaSpec> {
        IntEnaW::new(self, 0)
    }
}
#[doc = "Enables the RSA interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {}
