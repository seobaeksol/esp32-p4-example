#[doc = "Register `THRES_CTRL` reader"]
pub type R = crate::R<ThresCtrlSpec>;
#[doc = "Register `THRES_CTRL` writer"]
pub type W = crate::W<ThresCtrlSpec>;
#[doc = "Field `THRES_ALL_EN` reader - need_des"]
pub type ThresAllEnR = crate::BitReader;
#[doc = "Field `THRES_ALL_EN` writer - need_des"]
pub type ThresAllEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES3_EN` reader - need_des"]
pub type Thres3EnR = crate::BitReader;
#[doc = "Field `THRES3_EN` writer - need_des"]
pub type Thres3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES2_EN` reader - need_des"]
pub type Thres2EnR = crate::BitReader;
#[doc = "Field `THRES2_EN` writer - need_des"]
pub type Thres2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES1_EN` reader - need_des"]
pub type Thres1EnR = crate::BitReader;
#[doc = "Field `THRES1_EN` writer - need_des"]
pub type Thres1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES0_EN` reader - need_des"]
pub type Thres0EnR = crate::BitReader;
#[doc = "Field `THRES0_EN` writer - need_des"]
pub type Thres0EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn thres_all_en(&self) -> ThresAllEnR {
        ThresAllEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn thres3_en(&self) -> Thres3EnR {
        Thres3EnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn thres2_en(&self) -> Thres2EnR {
        Thres2EnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn thres1_en(&self) -> Thres1EnR {
        Thres1EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn thres0_en(&self) -> Thres0EnR {
        Thres0EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn thres_all_en(&mut self) -> ThresAllEnW<'_, ThresCtrlSpec> {
        ThresAllEnW::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn thres3_en(&mut self) -> Thres3EnW<'_, ThresCtrlSpec> {
        Thres3EnW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn thres2_en(&mut self) -> Thres2EnW<'_, ThresCtrlSpec> {
        Thres2EnW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn thres1_en(&mut self) -> Thres1EnW<'_, ThresCtrlSpec> {
        Thres1EnW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn thres0_en(&mut self) -> Thres0EnW<'_, ThresCtrlSpec> {
        Thres0EnW::new(self, 31)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThresCtrlSpec;
impl crate::RegisterSpec for ThresCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres_ctrl::R`](R) reader structure"]
impl crate::Readable for ThresCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`thres_ctrl::W`](W) writer structure"]
impl crate::Writable for ThresCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THRES_CTRL to value 0"]
impl crate::Resettable for ThresCtrlSpec {}
