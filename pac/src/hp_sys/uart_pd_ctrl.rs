#[doc = "Register `UART_PD_CTRL` reader"]
pub type R = crate::R<UartPdCtrlSpec>;
#[doc = "Register `UART_PD_CTRL` writer"]
pub type W = crate::W<UartPdCtrlSpec>;
#[doc = "Field `UART_MEM_FORCE_PD` reader - Set this bit to power down hp uart internal memory."]
pub type UartMemForcePdR = crate::BitReader;
#[doc = "Field `UART_MEM_FORCE_PD` writer - Set this bit to power down hp uart internal memory."]
pub type UartMemForcePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_MEM_FORCE_PU` reader - Set this bit to force power up hp uart internal memory"]
pub type UartMemForcePuR = crate::BitReader;
#[doc = "Field `UART_MEM_FORCE_PU` writer - Set this bit to force power up hp uart internal memory"]
pub type UartMemForcePuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to power down hp uart internal memory."]
    #[inline(always)]
    pub fn uart_mem_force_pd(&self) -> UartMemForcePdR {
        UartMemForcePdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force power up hp uart internal memory"]
    #[inline(always)]
    pub fn uart_mem_force_pu(&self) -> UartMemForcePuR {
        UartMemForcePuR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to power down hp uart internal memory."]
    #[inline(always)]
    pub fn uart_mem_force_pd(&mut self) -> UartMemForcePdW<'_, UartPdCtrlSpec> {
        UartMemForcePdW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force power up hp uart internal memory"]
    #[inline(always)]
    pub fn uart_mem_force_pu(&mut self) -> UartMemForcePuW<'_, UartPdCtrlSpec> {
        UartMemForcePuW::new(self, 1)
    }
}
#[doc = "ecc pd ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartPdCtrlSpec;
impl crate::RegisterSpec for UartPdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for UartPdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for UartPdCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_PD_CTRL to value 0x02"]
impl crate::Resettable for UartPdCtrlSpec {
    const RESET_VALUE: u32 = 0x02;
}
