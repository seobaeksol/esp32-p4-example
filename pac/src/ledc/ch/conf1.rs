#[doc = "Register `CONF1` reader"]
pub type R = crate::R<Conf1Spec>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<Conf1Spec>;
#[doc = "Field `DUTY_START` reader - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
pub type DutyStartR = crate::BitReader;
#[doc = "Field `DUTY_START` writer - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
pub type DutyStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
    #[inline(always)]
    pub fn duty_start(&self) -> DutyStartR {
        DutyStartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
    #[inline(always)]
    pub fn duty_start(&mut self) -> DutyStartW<'_, Conf1Spec> {
        DutyStartW::new(self, 31)
    }
}
#[doc = "Configuration register 1 for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Conf1Spec;
impl crate::RegisterSpec for Conf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for Conf1Spec {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for Conf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF1 to value 0"]
impl crate::Resettable for Conf1Spec {}
