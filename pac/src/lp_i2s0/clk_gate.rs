#[doc = "Register `CLK_GATE` reader"]
pub type R = crate::R<ClkGateSpec>;
#[doc = "Register `CLK_GATE` writer"]
pub type W = crate::W<ClkGateSpec>;
#[doc = "Field `CLK_EN` reader - set this bit to enable clock gate"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - set this bit to enable clock gate"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VAD_CG_FORCE_ON` reader - VAD clock gate force on register"]
pub type VadCgForceOnR = crate::BitReader;
#[doc = "Field `VAD_CG_FORCE_ON` writer - VAD clock gate force on register"]
pub type VadCgForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MEM_CG_FORCE_ON` reader - I2S rx mem clock gate force on register"]
pub type RxMemCgForceOnR = crate::BitReader;
#[doc = "Field `RX_MEM_CG_FORCE_ON` writer - I2S rx mem clock gate force on register"]
pub type RxMemCgForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_REG_CG_FORCE_ON` reader - I2S rx reg clock gate force on register"]
pub type RxRegCgForceOnR = crate::BitReader;
#[doc = "Field `RX_REG_CG_FORCE_ON` writer - I2S rx reg clock gate force on register"]
pub type RxRegCgForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set this bit to enable clock gate"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VAD clock gate force on register"]
    #[inline(always)]
    pub fn vad_cg_force_on(&self) -> VadCgForceOnR {
        VadCgForceOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2S rx mem clock gate force on register"]
    #[inline(always)]
    pub fn rx_mem_cg_force_on(&self) -> RxMemCgForceOnR {
        RxMemCgForceOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2S rx reg clock gate force on register"]
    #[inline(always)]
    pub fn rx_reg_cg_force_on(&self) -> RxRegCgForceOnR {
        RxRegCgForceOnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set this bit to enable clock gate"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, ClkGateSpec> {
        ClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - VAD clock gate force on register"]
    #[inline(always)]
    pub fn vad_cg_force_on(&mut self) -> VadCgForceOnW<'_, ClkGateSpec> {
        VadCgForceOnW::new(self, 1)
    }
    #[doc = "Bit 2 - I2S rx mem clock gate force on register"]
    #[inline(always)]
    pub fn rx_mem_cg_force_on(&mut self) -> RxMemCgForceOnW<'_, ClkGateSpec> {
        RxMemCgForceOnW::new(self, 2)
    }
    #[doc = "Bit 3 - I2S rx reg clock gate force on register"]
    #[inline(always)]
    pub fn rx_reg_cg_force_on(&mut self) -> RxRegCgForceOnW<'_, ClkGateSpec> {
        RxRegCgForceOnW::new(self, 3)
    }
}
#[doc = "Clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkGateSpec;
impl crate::RegisterSpec for ClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gate::R`](R) reader structure"]
impl crate::Readable for ClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_gate::W`](W) writer structure"]
impl crate::Writable for ClkGateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_GATE to value 0x0a"]
impl crate::Resettable for ClkGateSpec {
    const RESET_VALUE: u32 = 0x0a;
}
