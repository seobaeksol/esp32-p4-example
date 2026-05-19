#[doc = "Register `GAMMA_CTRL` reader"]
pub type R = crate::R<GammaCtrlSpec>;
#[doc = "Register `GAMMA_CTRL` writer"]
pub type W = crate::W<GammaCtrlSpec>;
#[doc = "Field `GAMMA_UPDATE` reader - Indicates that gamma register configuration is complete"]
pub type GammaUpdateR = crate::BitReader;
#[doc = "Field `GAMMA_UPDATE` writer - Indicates that gamma register configuration is complete"]
pub type GammaUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_B_LAST_CORRECT` reader - this bit configures enable of last b segment correcction. 0: disable, 1: enable"]
pub type GammaBLastCorrectR = crate::BitReader;
#[doc = "Field `GAMMA_B_LAST_CORRECT` writer - this bit configures enable of last b segment correcction. 0: disable, 1: enable"]
pub type GammaBLastCorrectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_G_LAST_CORRECT` reader - this bit configures enable of last g segment correcction. 0: disable, 1: enable"]
pub type GammaGLastCorrectR = crate::BitReader;
#[doc = "Field `GAMMA_G_LAST_CORRECT` writer - this bit configures enable of last g segment correcction. 0: disable, 1: enable"]
pub type GammaGLastCorrectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_R_LAST_CORRECT` reader - this bit configures enable of last r segment correcction. 0: disable, 1: enable"]
pub type GammaRLastCorrectR = crate::BitReader;
#[doc = "Field `GAMMA_R_LAST_CORRECT` writer - this bit configures enable of last r segment correcction. 0: disable, 1: enable"]
pub type GammaRLastCorrectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that gamma register configuration is complete"]
    #[inline(always)]
    pub fn gamma_update(&self) -> GammaUpdateR {
        GammaUpdateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit configures enable of last b segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_b_last_correct(&self) -> GammaBLastCorrectR {
        GammaBLastCorrectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit configures enable of last g segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_g_last_correct(&self) -> GammaGLastCorrectR {
        GammaGLastCorrectR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit configures enable of last r segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_r_last_correct(&self) -> GammaRLastCorrectR {
        GammaRLastCorrectR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that gamma register configuration is complete"]
    #[inline(always)]
    pub fn gamma_update(&mut self) -> GammaUpdateW<'_, GammaCtrlSpec> {
        GammaUpdateW::new(self, 0)
    }
    #[doc = "Bit 1 - this bit configures enable of last b segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_b_last_correct(&mut self) -> GammaBLastCorrectW<'_, GammaCtrlSpec> {
        GammaBLastCorrectW::new(self, 1)
    }
    #[doc = "Bit 2 - this bit configures enable of last g segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_g_last_correct(&mut self) -> GammaGLastCorrectW<'_, GammaCtrlSpec> {
        GammaGLastCorrectW::new(self, 2)
    }
    #[doc = "Bit 3 - this bit configures enable of last r segment correcction. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn gamma_r_last_correct(&mut self) -> GammaRLastCorrectW<'_, GammaCtrlSpec> {
        GammaRLastCorrectW::new(self, 3)
    }
}
#[doc = "gamma control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GammaCtrlSpec;
impl crate::RegisterSpec for GammaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_ctrl::R`](R) reader structure"]
impl crate::Readable for GammaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gamma_ctrl::W`](W) writer structure"]
impl crate::Writable for GammaCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAMMA_CTRL to value 0x0e"]
impl crate::Resettable for GammaCtrlSpec {
    const RESET_VALUE: u32 = 0x0e;
}
