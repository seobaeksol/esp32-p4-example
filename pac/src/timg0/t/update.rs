#[doc = "Register `UPDATE` reader"]
pub type R = crate::R<UpdateSpec>;
#[doc = "Register `UPDATE` writer"]
pub type W = crate::W<UpdateSpec>;
#[doc = "Field `UPDATE` reader - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
pub type UpdateR = crate::BitReader;
#[doc = "Field `UPDATE` writer - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
pub type UpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
    #[inline(always)]
    pub fn update(&self) -> UpdateR {
        UpdateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched."]
    #[inline(always)]
    pub fn update(&mut self) -> UpdateW<'_, UpdateSpec> {
        UpdateW::new(self, 31)
    }
}
#[doc = "Write to copy current timer value to TIMGn_T0_(LO/HI)_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`update::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`update::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpdateSpec;
impl crate::RegisterSpec for UpdateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`update::R`](R) reader structure"]
impl crate::Readable for UpdateSpec {}
#[doc = "`write(|w| ..)` method takes [`update::W`](W) writer structure"]
impl crate::Writable for UpdateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UPDATE to value 0"]
impl crate::Resettable for UpdateSpec {}
