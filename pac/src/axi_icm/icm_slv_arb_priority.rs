#[doc = "Register `ICM_SLV_ARB_PRIORITY` reader"]
pub type R = crate::R<IcmSlvArbPrioritySpec>;
#[doc = "Register `ICM_SLV_ARB_PRIORITY` writer"]
pub type W = crate::W<IcmSlvArbPrioritySpec>;
#[doc = "Field `L2MEM_PRIORITY` reader - "]
pub type L2memPriorityR = crate::FieldReader;
#[doc = "Field `L2MEM_PRIORITY` writer - "]
pub type L2memPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLASH_MSPI_PRIORITY` reader - "]
pub type FlashMspiPriorityR = crate::FieldReader;
#[doc = "Field `FLASH_MSPI_PRIORITY` writer - "]
pub type FlashMspiPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PSRAM_MSPI_PRIORITY` reader - "]
pub type PsramMspiPriorityR = crate::FieldReader;
#[doc = "Field `PSRAM_MSPI_PRIORITY` writer - "]
pub type PsramMspiPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LCD_PRIORITY` reader - "]
pub type LcdPriorityR = crate::FieldReader;
#[doc = "Field `LCD_PRIORITY` writer - "]
pub type LcdPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CAM_PRIORITY` reader - "]
pub type CamPriorityR = crate::FieldReader;
#[doc = "Field `CAM_PRIORITY` writer - "]
pub type CamPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn l2mem_priority(&self) -> L2memPriorityR {
        L2memPriorityR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn flash_mspi_priority(&self) -> FlashMspiPriorityR {
        FlashMspiPriorityR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn psram_mspi_priority(&self) -> PsramMspiPriorityR {
        PsramMspiPriorityR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn lcd_priority(&self) -> LcdPriorityR {
        LcdPriorityR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn cam_priority(&self) -> CamPriorityR {
        CamPriorityR::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn l2mem_priority(&mut self) -> L2memPriorityW<'_, IcmSlvArbPrioritySpec> {
        L2memPriorityW::new(self, 3)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn flash_mspi_priority(&mut self) -> FlashMspiPriorityW<'_, IcmSlvArbPrioritySpec> {
        FlashMspiPriorityW::new(self, 12)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn psram_mspi_priority(&mut self) -> PsramMspiPriorityW<'_, IcmSlvArbPrioritySpec> {
        PsramMspiPriorityW::new(self, 15)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn lcd_priority(&mut self) -> LcdPriorityW<'_, IcmSlvArbPrioritySpec> {
        LcdPriorityW::new(self, 18)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn cam_priority(&mut self) -> CamPriorityW<'_, IcmSlvArbPrioritySpec> {
        CamPriorityW::new(self, 21)
    }
}
#[doc = "Slave arbitration priority\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_slv_arb_priority::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_slv_arb_priority::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmSlvArbPrioritySpec;
impl crate::RegisterSpec for IcmSlvArbPrioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_slv_arb_priority::R`](R) reader structure"]
impl crate::Readable for IcmSlvArbPrioritySpec {}
#[doc = "`write(|w| ..)` method takes [`icm_slv_arb_priority::W`](W) writer structure"]
impl crate::Writable for IcmSlvArbPrioritySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_SLV_ARB_PRIORITY to value 0"]
impl crate::Resettable for IcmSlvArbPrioritySpec {}
