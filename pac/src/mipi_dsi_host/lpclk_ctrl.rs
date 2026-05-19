#[doc = "Register `LPCLK_CTRL` reader"]
pub type R = crate::R<LpclkCtrlSpec>;
#[doc = "Register `LPCLK_CTRL` writer"]
pub type W = crate::W<LpclkCtrlSpec>;
#[doc = "Field `PHY_TXREQUESTCLKHS` reader - NA"]
pub type PhyTxrequestclkhsR = crate::BitReader;
#[doc = "Field `PHY_TXREQUESTCLKHS` writer - NA"]
pub type PhyTxrequestclkhsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CLKLANE_CTRL` reader - NA"]
pub type AutoClklaneCtrlR = crate::BitReader;
#[doc = "Field `AUTO_CLKLANE_CTRL` writer - NA"]
pub type AutoClklaneCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_txrequestclkhs(&self) -> PhyTxrequestclkhsR {
        PhyTxrequestclkhsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn auto_clklane_ctrl(&self) -> AutoClklaneCtrlR {
        AutoClklaneCtrlR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_txrequestclkhs(&mut self) -> PhyTxrequestclkhsW<'_, LpclkCtrlSpec> {
        PhyTxrequestclkhsW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn auto_clklane_ctrl(&mut self) -> AutoClklaneCtrlW<'_, LpclkCtrlSpec> {
        AutoClklaneCtrlW::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lpclk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpclk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpclkCtrlSpec;
impl crate::RegisterSpec for LpclkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpclk_ctrl::R`](R) reader structure"]
impl crate::Readable for LpclkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lpclk_ctrl::W`](W) writer structure"]
impl crate::Writable for LpclkCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPCLK_CTRL to value 0"]
impl crate::Resettable for LpclkCtrlSpec {}
