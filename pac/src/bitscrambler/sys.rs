#[doc = "Register `SYS` reader"]
pub type R = crate::R<SysSpec>;
#[doc = "Register `SYS` writer"]
pub type W = crate::W<SysSpec>;
#[doc = "Field `LOOP_MODE` reader - write this bit to set the bitscrambler tx loop back to DMA rx"]
pub type LoopModeR = crate::BitReader;
#[doc = "Field `LOOP_MODE` writer - write this bit to set the bitscrambler tx loop back to DMA rx"]
pub type LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Reserved"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Reserved"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write this bit to set the bitscrambler tx loop back to DMA rx"]
    #[inline(always)]
    pub fn loop_mode(&self) -> LoopModeR {
        LoopModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write this bit to set the bitscrambler tx loop back to DMA rx"]
    #[inline(always)]
    pub fn loop_mode(&mut self) -> LoopModeW<'_, SysSpec> {
        LoopModeW::new(self, 0)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, SysSpec> {
        ClkEnW::new(self, 31)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`sys::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSpec;
impl crate::RegisterSpec for SysSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys::R`](R) reader structure"]
impl crate::Readable for SysSpec {}
#[doc = "`write(|w| ..)` method takes [`sys::W`](W) writer structure"]
impl crate::Writable for SysSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS to value 0"]
impl crate::Resettable for SysSpec {}
