#[doc = "Register `LINK_SWITCH_STATE` reader"]
pub type R = crate::R<LinkSwitchStateSpec>;
#[doc = "Register `LINK_SWITCH_STATE` writer"]
pub type W = crate::W<LinkSwitchStateSpec>;
#[doc = "Field `CH0` reader - The register that confirm ch dscr switch success"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - The register that confirm ch dscr switch success"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - The register that confirm ch dscr switch success"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - The register that confirm ch dscr switch success"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - The register that confirm ch dscr switch success"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - The register that confirm ch dscr switch success"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, LinkSwitchStateSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, LinkSwitchStateSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, LinkSwitchStateSpec> {
        Ch2W::new(self, 2)
    }
}
#[doc = "Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`link_switch_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link_switch_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinkSwitchStateSpec;
impl crate::RegisterSpec for LinkSwitchStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_switch_state::R`](R) reader structure"]
impl crate::Readable for LinkSwitchStateSpec {}
#[doc = "`write(|w| ..)` method takes [`link_switch_state::W`](W) writer structure"]
impl crate::Writable for LinkSwitchStateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK_SWITCH_STATE to value 0"]
impl crate::Resettable for LinkSwitchStateSpec {}
