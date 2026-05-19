#[doc = "Register `INT_IP%s` reader"]
pub type R = crate::R<IntIpSpec>;
#[doc = "Register `INT_IP%s` writer"]
pub type W = crate::W<IntIpSpec>;
#[doc = "Field `INT_IP` reader - Interrupt pending bit."]
pub type IntIpR = crate::BitReader;
#[doc = "Field `INT_IP` writer - Interrupt pending bit."]
pub type IntIpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt pending bit."]
    #[inline(always)]
    pub fn int_ip(&self) -> IntIpR {
        IntIpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt pending bit."]
    #[inline(always)]
    pub fn int_ip(&mut self) -> IntIpW<'_, IntIpSpec> {
        IntIpW::new(self, 0)
    }
}
#[doc = "Interrupt pending register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntIpSpec;
impl crate::RegisterSpec for IntIpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`int_ip::R`](R) reader structure"]
impl crate::Readable for IntIpSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ip::W`](W) writer structure"]
impl crate::Writable for IntIpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_IP%s to value 0"]
impl crate::Resettable for IntIpSpec {}
