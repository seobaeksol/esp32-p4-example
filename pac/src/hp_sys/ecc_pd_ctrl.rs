#[doc = "Register `ECC_PD_CTRL` reader"]
pub type R = crate::R<EccPdCtrlSpec>;
#[doc = "Register `ECC_PD_CTRL` writer"]
pub type W = crate::W<EccPdCtrlSpec>;
#[doc = "Field `ECC_MEM_FORCE_PD` reader - Set this bit to power down ecc internal memory."]
pub type EccMemForcePdR = crate::BitReader;
#[doc = "Field `ECC_MEM_FORCE_PD` writer - Set this bit to power down ecc internal memory."]
pub type EccMemForcePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_MEM_FORCE_PU` reader - Set this bit to force power up ecc internal memory"]
pub type EccMemForcePuR = crate::BitReader;
#[doc = "Field `ECC_MEM_FORCE_PU` writer - Set this bit to force power up ecc internal memory"]
pub type EccMemForcePuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_MEM_PD` reader - Set this bit to force power down ecc internal memory."]
pub type EccMemPdR = crate::BitReader;
#[doc = "Field `ECC_MEM_PD` writer - Set this bit to force power down ecc internal memory."]
pub type EccMemPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to power down ecc internal memory."]
    #[inline(always)]
    pub fn ecc_mem_force_pd(&self) -> EccMemForcePdR {
        EccMemForcePdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force power up ecc internal memory"]
    #[inline(always)]
    pub fn ecc_mem_force_pu(&self) -> EccMemForcePuR {
        EccMemForcePuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power down ecc internal memory."]
    #[inline(always)]
    pub fn ecc_mem_pd(&self) -> EccMemPdR {
        EccMemPdR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to power down ecc internal memory."]
    #[inline(always)]
    pub fn ecc_mem_force_pd(&mut self) -> EccMemForcePdW<'_, EccPdCtrlSpec> {
        EccMemForcePdW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force power up ecc internal memory"]
    #[inline(always)]
    pub fn ecc_mem_force_pu(&mut self) -> EccMemForcePuW<'_, EccPdCtrlSpec> {
        EccMemForcePuW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power down ecc internal memory."]
    #[inline(always)]
    pub fn ecc_mem_pd(&mut self) -> EccMemPdW<'_, EccPdCtrlSpec> {
        EccMemPdW::new(self, 2)
    }
}
#[doc = "ecc pd ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccPdCtrlSpec;
impl crate::RegisterSpec for EccPdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for EccPdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for EccPdCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECC_PD_CTRL to value 0x02"]
impl crate::Resettable for EccPdCtrlSpec {
    const RESET_VALUE: u32 = 0x02;
}
