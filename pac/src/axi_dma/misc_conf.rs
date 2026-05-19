#[doc = "Register `MISC_CONF` reader"]
pub type R = crate::R<MiscConfSpec>;
#[doc = "Register `MISC_CONF` writer"]
pub type W = crate::W<MiscConfSpec>;
#[doc = "Field `AXIM_RST_WR_INTER` reader - Set this bit then clear this bit to reset the internal axi_wr FSM."]
pub type AximRstWrInterR = crate::BitReader;
#[doc = "Field `AXIM_RST_WR_INTER` writer - Set this bit then clear this bit to reset the internal axi_wr FSM."]
pub type AximRstWrInterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIM_RST_RD_INTER` reader - Set this bit then clear this bit to reset the internal axi_rd FSM."]
pub type AximRstRdInterR = crate::BitReader;
#[doc = "Field `AXIM_RST_RD_INTER` writer - Set this bit then clear this bit to reset the internal axi_rd FSM."]
pub type AximRstRdInterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_PRI_DIS` reader - Set this bit to disable priority arbitration function."]
pub type ArbPriDisR = crate::BitReader;
#[doc = "Field `ARB_PRI_DIS` writer - Set this bit to disable priority arbitration function."]
pub type ArbPriDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit then clear this bit to reset the internal axi_wr FSM."]
    #[inline(always)]
    pub fn axim_rst_wr_inter(&self) -> AximRstWrInterR {
        AximRstWrInterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit then clear this bit to reset the internal axi_rd FSM."]
    #[inline(always)]
    pub fn axim_rst_rd_inter(&self) -> AximRstRdInterR {
        AximRstRdInterR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to disable priority arbitration function."]
    #[inline(always)]
    pub fn arb_pri_dis(&self) -> ArbPriDisR {
        ArbPriDisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit then clear this bit to reset the internal axi_wr FSM."]
    #[inline(always)]
    pub fn axim_rst_wr_inter(&mut self) -> AximRstWrInterW<'_, MiscConfSpec> {
        AximRstWrInterW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit then clear this bit to reset the internal axi_rd FSM."]
    #[inline(always)]
    pub fn axim_rst_rd_inter(&mut self) -> AximRstRdInterW<'_, MiscConfSpec> {
        AximRstRdInterW::new(self, 1)
    }
    #[doc = "Bit 3 - Set this bit to disable priority arbitration function."]
    #[inline(always)]
    pub fn arb_pri_dis(&mut self) -> ArbPriDisW<'_, MiscConfSpec> {
        ArbPriDisW::new(self, 3)
    }
    #[doc = "Bit 4 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, MiscConfSpec> {
        ClkEnW::new(self, 4)
    }
}
#[doc = "MISC register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscConfSpec;
impl crate::RegisterSpec for MiscConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_conf::R`](R) reader structure"]
impl crate::Readable for MiscConfSpec {}
#[doc = "`write(|w| ..)` method takes [`misc_conf::W`](W) writer structure"]
impl crate::Writable for MiscConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC_CONF to value 0"]
impl crate::Resettable for MiscConfSpec {}
