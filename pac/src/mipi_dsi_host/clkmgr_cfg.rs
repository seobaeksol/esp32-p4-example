#[doc = "Register `CLKMGR_CFG` reader"]
pub type R = crate::R<ClkmgrCfgSpec>;
#[doc = "Register `CLKMGR_CFG` writer"]
pub type W = crate::W<ClkmgrCfgSpec>;
#[doc = "Field `TX_ESC_CLK_DIVISION` reader - NA"]
pub type TxEscClkDivisionR = crate::FieldReader;
#[doc = "Field `TX_ESC_CLK_DIVISION` writer - NA"]
pub type TxEscClkDivisionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TO_CLK_DIVISION` reader - NA"]
pub type ToClkDivisionR = crate::FieldReader;
#[doc = "Field `TO_CLK_DIVISION` writer - NA"]
pub type ToClkDivisionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn tx_esc_clk_division(&self) -> TxEscClkDivisionR {
        TxEscClkDivisionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn to_clk_division(&self) -> ToClkDivisionR {
        ToClkDivisionR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn tx_esc_clk_division(&mut self) -> TxEscClkDivisionW<'_, ClkmgrCfgSpec> {
        TxEscClkDivisionW::new(self, 0)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn to_clk_division(&mut self) -> ToClkDivisionW<'_, ClkmgrCfgSpec> {
        ToClkDivisionW::new(self, 8)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`clkmgr_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkmgr_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkmgrCfgSpec;
impl crate::RegisterSpec for ClkmgrCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkmgr_cfg::R`](R) reader structure"]
impl crate::Readable for ClkmgrCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clkmgr_cfg::W`](W) writer structure"]
impl crate::Writable for ClkmgrCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKMGR_CFG to value 0"]
impl crate::Resettable for ClkmgrCfgSpec {}
