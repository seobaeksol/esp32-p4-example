#[doc = "Register `MEM_CONF` reader"]
pub type R = crate::R<MemConfSpec>;
#[doc = "Register `MEM_CONF` writer"]
pub type W = crate::W<MemConfSpec>;
#[doc = "Field `USB_MEM_PD` reader - 1: power down usb memory."]
pub type UsbMemPdR = crate::BitReader;
#[doc = "Field `USB_MEM_PD` writer - 1: power down usb memory."]
pub type UsbMemPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_MEM_CLK_EN` reader - 1: Force clock on for usb memory."]
pub type UsbMemClkEnR = crate::BitReader;
#[doc = "Field `USB_MEM_CLK_EN` writer - 1: Force clock on for usb memory."]
pub type UsbMemClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: power down usb memory."]
    #[inline(always)]
    pub fn usb_mem_pd(&self) -> UsbMemPdR {
        UsbMemPdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Force clock on for usb memory."]
    #[inline(always)]
    pub fn usb_mem_clk_en(&self) -> UsbMemClkEnR {
        UsbMemClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: power down usb memory."]
    #[inline(always)]
    pub fn usb_mem_pd(&mut self) -> UsbMemPdW<'_, MemConfSpec> {
        UsbMemPdW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Force clock on for usb memory."]
    #[inline(always)]
    pub fn usb_mem_clk_en(&mut self) -> UsbMemClkEnW<'_, MemConfSpec> {
        UsbMemClkEnW::new(self, 1)
    }
}
#[doc = "Memory power control\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemConfSpec;
impl crate::RegisterSpec for MemConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_conf::R`](R) reader structure"]
impl crate::Readable for MemConfSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_conf::W`](W) writer structure"]
impl crate::Writable for MemConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CONF to value 0x02"]
impl crate::Resettable for MemConfSpec {
    const RESET_VALUE: u32 = 0x02;
}
