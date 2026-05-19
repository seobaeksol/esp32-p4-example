#[doc = "Register `DT_CFG` reader"]
pub type R = crate::R<DtCfgSpec>;
#[doc = "Register `DT_CFG` writer"]
pub type W = crate::W<DtCfgSpec>;
#[doc = "Field `FED_UPMETHOD` reader - Configures update method for FED (Falling edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type FedUpmethodR = crate::FieldReader;
#[doc = "Field `FED_UPMETHOD` writer - Configures update method for FED (Falling edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type FedUpmethodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RED_UPMETHOD` reader - Configures update method for RED (rising edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type RedUpmethodR = crate::FieldReader;
#[doc = "Field `RED_UPMETHOD` writer - Configures update method for RED (rising edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type RedUpmethodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEB_MODE` reader - Configures S8 in table, dual-edge B mode.\\\\0: fed/red take effect on different path separately\\\\1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub type DebModeR = crate::BitReader;
#[doc = "Field `DEB_MODE` writer - Configures S8 in table, dual-edge B mode.\\\\0: fed/red take effect on different path separately\\\\1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub type DebModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_OUTSWAP` reader - Configures S6 in table."]
pub type AOutswapR = crate::BitReader;
#[doc = "Field `A_OUTSWAP` writer - Configures S6 in table."]
pub type AOutswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_OUTSWAP` reader - Configures S7 in table."]
pub type BOutswapR = crate::BitReader;
#[doc = "Field `B_OUTSWAP` writer - Configures S7 in table."]
pub type BOutswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_INSEL` reader - Configures S4 in table."]
pub type RedInselR = crate::BitReader;
#[doc = "Field `RED_INSEL` writer - Configures S4 in table."]
pub type RedInselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FED_INSEL` reader - Configures S5 in table."]
pub type FedInselR = crate::BitReader;
#[doc = "Field `FED_INSEL` writer - Configures S5 in table."]
pub type FedInselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_OUTINVERT` reader - Configures S2 in table."]
pub type RedOutinvertR = crate::BitReader;
#[doc = "Field `RED_OUTINVERT` writer - Configures S2 in table."]
pub type RedOutinvertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FED_OUTINVERT` reader - Configures S3 in table."]
pub type FedOutinvertR = crate::BitReader;
#[doc = "Field `FED_OUTINVERT` writer - Configures S3 in table."]
pub type FedOutinvertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_OUTBYPASS` reader - Configures S1 in table."]
pub type AOutbypassR = crate::BitReader;
#[doc = "Field `A_OUTBYPASS` writer - Configures S1 in table."]
pub type AOutbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_OUTBYPASS` reader - Configures S0 in table."]
pub type BOutbypassR = crate::BitReader;
#[doc = "Field `B_OUTBYPASS` writer - Configures S0 in table."]
pub type BOutbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SEL` reader - Configures dead time generator %s clock selection.\\\\0: PWM_clk\\\\1: PT_clk"]
pub type ClkSelR = crate::BitReader;
#[doc = "Field `CLK_SEL` writer - Configures dead time generator %s clock selection.\\\\0: PWM_clk\\\\1: PT_clk"]
pub type ClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Configures update method for FED (Falling edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn fed_upmethod(&self) -> FedUpmethodR {
        FedUpmethodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configures update method for RED (rising edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn red_upmethod(&self) -> RedUpmethodR {
        RedUpmethodR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Configures S8 in table, dual-edge B mode.\\\\0: fed/red take effect on different path separately\\\\1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    pub fn deb_mode(&self) -> DebModeR {
        DebModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures S6 in table."]
    #[inline(always)]
    pub fn a_outswap(&self) -> AOutswapR {
        AOutswapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures S7 in table."]
    #[inline(always)]
    pub fn b_outswap(&self) -> BOutswapR {
        BOutswapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures S4 in table."]
    #[inline(always)]
    pub fn red_insel(&self) -> RedInselR {
        RedInselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures S5 in table."]
    #[inline(always)]
    pub fn fed_insel(&self) -> FedInselR {
        FedInselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures S2 in table."]
    #[inline(always)]
    pub fn red_outinvert(&self) -> RedOutinvertR {
        RedOutinvertR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures S3 in table."]
    #[inline(always)]
    pub fn fed_outinvert(&self) -> FedOutinvertR {
        FedOutinvertR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures S1 in table."]
    #[inline(always)]
    pub fn a_outbypass(&self) -> AOutbypassR {
        AOutbypassR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configures S0 in table."]
    #[inline(always)]
    pub fn b_outbypass(&self) -> BOutbypassR {
        BOutbypassR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures dead time generator %s clock selection.\\\\0: PWM_clk\\\\1: PT_clk"]
    #[inline(always)]
    pub fn clk_sel(&self) -> ClkSelR {
        ClkSelR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures update method for FED (Falling edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn fed_upmethod(&mut self) -> FedUpmethodW<'_, DtCfgSpec> {
        FedUpmethodW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configures update method for RED (rising edge delay) active register.\\\\0: Immediate\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn red_upmethod(&mut self) -> RedUpmethodW<'_, DtCfgSpec> {
        RedUpmethodW::new(self, 4)
    }
    #[doc = "Bit 8 - Configures S8 in table, dual-edge B mode.\\\\0: fed/red take effect on different path separately\\\\1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    pub fn deb_mode(&mut self) -> DebModeW<'_, DtCfgSpec> {
        DebModeW::new(self, 8)
    }
    #[doc = "Bit 9 - Configures S6 in table."]
    #[inline(always)]
    pub fn a_outswap(&mut self) -> AOutswapW<'_, DtCfgSpec> {
        AOutswapW::new(self, 9)
    }
    #[doc = "Bit 10 - Configures S7 in table."]
    #[inline(always)]
    pub fn b_outswap(&mut self) -> BOutswapW<'_, DtCfgSpec> {
        BOutswapW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures S4 in table."]
    #[inline(always)]
    pub fn red_insel(&mut self) -> RedInselW<'_, DtCfgSpec> {
        RedInselW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures S5 in table."]
    #[inline(always)]
    pub fn fed_insel(&mut self) -> FedInselW<'_, DtCfgSpec> {
        FedInselW::new(self, 12)
    }
    #[doc = "Bit 13 - Configures S2 in table."]
    #[inline(always)]
    pub fn red_outinvert(&mut self) -> RedOutinvertW<'_, DtCfgSpec> {
        RedOutinvertW::new(self, 13)
    }
    #[doc = "Bit 14 - Configures S3 in table."]
    #[inline(always)]
    pub fn fed_outinvert(&mut self) -> FedOutinvertW<'_, DtCfgSpec> {
        FedOutinvertW::new(self, 14)
    }
    #[doc = "Bit 15 - Configures S1 in table."]
    #[inline(always)]
    pub fn a_outbypass(&mut self) -> AOutbypassW<'_, DtCfgSpec> {
        AOutbypassW::new(self, 15)
    }
    #[doc = "Bit 16 - Configures S0 in table."]
    #[inline(always)]
    pub fn b_outbypass(&mut self) -> BOutbypassW<'_, DtCfgSpec> {
        BOutbypassW::new(self, 16)
    }
    #[doc = "Bit 17 - Configures dead time generator %s clock selection.\\\\0: PWM_clk\\\\1: PT_clk"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> ClkSelW<'_, DtCfgSpec> {
        ClkSelW::new(self, 17)
    }
}
#[doc = "Dead time configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtCfgSpec;
impl crate::RegisterSpec for DtCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt_cfg::R`](R) reader structure"]
impl crate::Readable for DtCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dt_cfg::W`](W) writer structure"]
impl crate::Writable for DtCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT_CFG to value 0x0001_8000"]
impl crate::Resettable for DtCfgSpec {
    const RESET_VALUE: u32 = 0x0001_8000;
}
