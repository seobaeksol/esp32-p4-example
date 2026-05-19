#[doc = "Register `TCM_RAM_WRR_CONFIG` reader"]
pub type R = crate::R<TcmRamWrrConfigSpec>;
#[doc = "Register `TCM_RAM_WRR_CONFIG` writer"]
pub type W = crate::W<TcmRamWrrConfigSpec>;
#[doc = "Field `REG_TCM_RAM_IBUS0_WT` reader - weight value of ibus0"]
pub type RegTcmRamIbus0WtR = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_IBUS0_WT` writer - weight value of ibus0"]
pub type RegTcmRamIbus0WtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_IBUS1_WT` reader - weight value of ibus1"]
pub type RegTcmRamIbus1WtR = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_IBUS1_WT` writer - weight value of ibus1"]
pub type RegTcmRamIbus1WtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_IBUS2_WT` reader - weight value of ibus2"]
pub type RegTcmRamIbus2WtR = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_IBUS2_WT` writer - weight value of ibus2"]
pub type RegTcmRamIbus2WtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_IBUS3_WT` reader - weight value of ibus3"]
pub type RegTcmRamIbus3WtR = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_IBUS3_WT` writer - weight value of ibus3"]
pub type RegTcmRamIbus3WtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_DBUS0_WT` reader - weight value of dbus0"]
pub type RegTcmRamDbus0WtR = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_DBUS0_WT` writer - weight value of dbus0"]
pub type RegTcmRamDbus0WtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_DBUS1_WT` reader - weight value of dbus1"]
pub type RegTcmRamDbus1WtR = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_DBUS1_WT` writer - weight value of dbus1"]
pub type RegTcmRamDbus1WtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_DBUS2_WT` reader - weight value of dbus2"]
pub type RegTcmRamDbus2WtR = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_DBUS2_WT` writer - weight value of dbus2"]
pub type RegTcmRamDbus2WtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_DBUS3_WT` reader - weight value of dbus3"]
pub type RegTcmRamDbus3WtR = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_DBUS3_WT` writer - weight value of dbus3"]
pub type RegTcmRamDbus3WtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_DMA_WT` reader - weight value of dma"]
pub type RegTcmRamDmaWtR = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_DMA_WT` writer - weight value of dma"]
pub type RegTcmRamDmaWtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_WRR_HIGH` reader - enable weighted round robin arbitration"]
pub type RegTcmRamWrrHighR = crate::BitReader;
#[doc = "Field `REG_TCM_RAM_WRR_HIGH` writer - enable weighted round robin arbitration"]
pub type RegTcmRamWrrHighW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - weight value of ibus0"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus0_wt(&self) -> RegTcmRamIbus0WtR {
        RegTcmRamIbus0WtR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - weight value of ibus1"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus1_wt(&self) -> RegTcmRamIbus1WtR {
        RegTcmRamIbus1WtR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - weight value of ibus2"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus2_wt(&self) -> RegTcmRamIbus2WtR {
        RegTcmRamIbus2WtR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - weight value of ibus3"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus3_wt(&self) -> RegTcmRamIbus3WtR {
        RegTcmRamIbus3WtR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - weight value of dbus0"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus0_wt(&self) -> RegTcmRamDbus0WtR {
        RegTcmRamDbus0WtR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - weight value of dbus1"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus1_wt(&self) -> RegTcmRamDbus1WtR {
        RegTcmRamDbus1WtR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - weight value of dbus2"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus2_wt(&self) -> RegTcmRamDbus2WtR {
        RegTcmRamDbus2WtR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - weight value of dbus3"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus3_wt(&self) -> RegTcmRamDbus3WtR {
        RegTcmRamDbus3WtR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - weight value of dma"]
    #[inline(always)]
    pub fn reg_tcm_ram_dma_wt(&self) -> RegTcmRamDmaWtR {
        RegTcmRamDmaWtR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - enable weighted round robin arbitration"]
    #[inline(always)]
    pub fn reg_tcm_ram_wrr_high(&self) -> RegTcmRamWrrHighR {
        RegTcmRamWrrHighR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - weight value of ibus0"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus0_wt(&mut self) -> RegTcmRamIbus0WtW<'_, TcmRamWrrConfigSpec> {
        RegTcmRamIbus0WtW::new(self, 0)
    }
    #[doc = "Bits 3:5 - weight value of ibus1"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus1_wt(&mut self) -> RegTcmRamIbus1WtW<'_, TcmRamWrrConfigSpec> {
        RegTcmRamIbus1WtW::new(self, 3)
    }
    #[doc = "Bits 6:8 - weight value of ibus2"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus2_wt(&mut self) -> RegTcmRamIbus2WtW<'_, TcmRamWrrConfigSpec> {
        RegTcmRamIbus2WtW::new(self, 6)
    }
    #[doc = "Bits 9:11 - weight value of ibus3"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus3_wt(&mut self) -> RegTcmRamIbus3WtW<'_, TcmRamWrrConfigSpec> {
        RegTcmRamIbus3WtW::new(self, 9)
    }
    #[doc = "Bits 12:14 - weight value of dbus0"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus0_wt(&mut self) -> RegTcmRamDbus0WtW<'_, TcmRamWrrConfigSpec> {
        RegTcmRamDbus0WtW::new(self, 12)
    }
    #[doc = "Bits 15:17 - weight value of dbus1"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus1_wt(&mut self) -> RegTcmRamDbus1WtW<'_, TcmRamWrrConfigSpec> {
        RegTcmRamDbus1WtW::new(self, 15)
    }
    #[doc = "Bits 18:20 - weight value of dbus2"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus2_wt(&mut self) -> RegTcmRamDbus2WtW<'_, TcmRamWrrConfigSpec> {
        RegTcmRamDbus2WtW::new(self, 18)
    }
    #[doc = "Bits 21:23 - weight value of dbus3"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus3_wt(&mut self) -> RegTcmRamDbus3WtW<'_, TcmRamWrrConfigSpec> {
        RegTcmRamDbus3WtW::new(self, 21)
    }
    #[doc = "Bits 24:26 - weight value of dma"]
    #[inline(always)]
    pub fn reg_tcm_ram_dma_wt(&mut self) -> RegTcmRamDmaWtW<'_, TcmRamWrrConfigSpec> {
        RegTcmRamDmaWtW::new(self, 24)
    }
    #[doc = "Bit 31 - enable weighted round robin arbitration"]
    #[inline(always)]
    pub fn reg_tcm_ram_wrr_high(&mut self) -> RegTcmRamWrrHighW<'_, TcmRamWrrConfigSpec> {
        RegTcmRamWrrHighW::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_ram_wrr_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_ram_wrr_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmRamWrrConfigSpec;
impl crate::RegisterSpec for TcmRamWrrConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_ram_wrr_config::R`](R) reader structure"]
impl crate::Readable for TcmRamWrrConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`tcm_ram_wrr_config::W`](W) writer structure"]
impl crate::Writable for TcmRamWrrConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_RAM_WRR_CONFIG to value 0x826e_d93f"]
impl crate::Resettable for TcmRamWrrConfigSpec {
    const RESET_VALUE: u32 = 0x826e_d93f;
}
