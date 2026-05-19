#[doc = "Register `LP_AONCLKRST_HP_USB_CLKRST_CTRL0` reader"]
pub type R = crate::R<LpAonclkrstHpUsbClkrstCtrl0Spec>;
#[doc = "Register `LP_AONCLKRST_HP_USB_CLKRST_CTRL0` writer"]
pub type W = crate::W<LpAonclkrstHpUsbClkrstCtrl0Spec>;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_SLEEP_MODE` reader - unused."]
pub type LpAonclkrstUsbOtg20SleepModeR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_SLEEP_MODE` writer - unused."]
pub type LpAonclkrstUsbOtg20SleepModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN` reader - unused."]
pub type LpAonclkrstUsbOtg20BkSysClkEnR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN` writer - unused."]
pub type LpAonclkrstUsbOtg20BkSysClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_SLEEP_MODE` reader - unused."]
pub type LpAonclkrstUsbOtg11SleepModeR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_SLEEP_MODE` writer - unused."]
pub type LpAonclkrstUsbOtg11SleepModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN` reader - unused."]
pub type LpAonclkrstUsbOtg11BkSysClkEnR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN` writer - unused."]
pub type LpAonclkrstUsbOtg11BkSysClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_48M_CLK_EN` reader - usb otg11 fs phy clock enable."]
pub type LpAonclkrstUsbOtg11_48mClkEnR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_48M_CLK_EN` writer - usb otg11 fs phy clock enable."]
pub type LpAonclkrstUsbOtg11_48mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_DEVICE_48M_CLK_EN` reader - usb device fs phy clock enable."]
pub type LpAonclkrstUsbDevice48mClkEnR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_DEVICE_48M_CLK_EN` writer - usb device fs phy clock enable."]
pub type LpAonclkrstUsbDevice48mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_48M_DIV_NUM` reader - usb 480m to 25m divide number."]
pub type LpAonclkrstUsb48mDivNumR = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_USB_48M_DIV_NUM` writer - usb 480m to 25m divide number."]
pub type LpAonclkrstUsb48mDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_AONCLKRST_USB_25M_DIV_NUM` reader - usb 500m to 25m divide number."]
pub type LpAonclkrstUsb25mDivNumR = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_USB_25M_DIV_NUM` writer - usb 500m to 25m divide number."]
pub type LpAonclkrstUsb25mDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_AONCLKRST_USB_12M_DIV_NUM` reader - usb 480m to 12m divide number."]
pub type LpAonclkrstUsb12mDivNumR = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_USB_12M_DIV_NUM` writer - usb 480m to 12m divide number."]
pub type LpAonclkrstUsb12mDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_sleep_mode(&self) -> LpAonclkrstUsbOtg20SleepModeR {
        LpAonclkrstUsbOtg20SleepModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_bk_sys_clk_en(&self) -> LpAonclkrstUsbOtg20BkSysClkEnR {
        LpAonclkrstUsbOtg20BkSysClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_sleep_mode(&self) -> LpAonclkrstUsbOtg11SleepModeR {
        LpAonclkrstUsbOtg11SleepModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_bk_sys_clk_en(&self) -> LpAonclkrstUsbOtg11BkSysClkEnR {
        LpAonclkrstUsbOtg11BkSysClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - usb otg11 fs phy clock enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_48m_clk_en(&self) -> LpAonclkrstUsbOtg11_48mClkEnR {
        LpAonclkrstUsbOtg11_48mClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - usb device fs phy clock enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_device_48m_clk_en(&self) -> LpAonclkrstUsbDevice48mClkEnR {
        LpAonclkrstUsbDevice48mClkEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - usb 480m to 25m divide number."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_48m_div_num(&self) -> LpAonclkrstUsb48mDivNumR {
        LpAonclkrstUsb48mDivNumR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:21 - usb 500m to 25m divide number."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_25m_div_num(&self) -> LpAonclkrstUsb25mDivNumR {
        LpAonclkrstUsb25mDivNumR::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:29 - usb 480m to 12m divide number."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_12m_div_num(&self) -> LpAonclkrstUsb12mDivNumR {
        LpAonclkrstUsb12mDivNumR::new(((self.bits >> 22) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_sleep_mode(
        &mut self,
    ) -> LpAonclkrstUsbOtg20SleepModeW<'_, LpAonclkrstHpUsbClkrstCtrl0Spec> {
        LpAonclkrstUsbOtg20SleepModeW::new(self, 0)
    }
    #[doc = "Bit 1 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_bk_sys_clk_en(
        &mut self,
    ) -> LpAonclkrstUsbOtg20BkSysClkEnW<'_, LpAonclkrstHpUsbClkrstCtrl0Spec> {
        LpAonclkrstUsbOtg20BkSysClkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_sleep_mode(
        &mut self,
    ) -> LpAonclkrstUsbOtg11SleepModeW<'_, LpAonclkrstHpUsbClkrstCtrl0Spec> {
        LpAonclkrstUsbOtg11SleepModeW::new(self, 2)
    }
    #[doc = "Bit 3 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_bk_sys_clk_en(
        &mut self,
    ) -> LpAonclkrstUsbOtg11BkSysClkEnW<'_, LpAonclkrstHpUsbClkrstCtrl0Spec> {
        LpAonclkrstUsbOtg11BkSysClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - usb otg11 fs phy clock enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_48m_clk_en(
        &mut self,
    ) -> LpAonclkrstUsbOtg11_48mClkEnW<'_, LpAonclkrstHpUsbClkrstCtrl0Spec> {
        LpAonclkrstUsbOtg11_48mClkEnW::new(self, 4)
    }
    #[doc = "Bit 5 - usb device fs phy clock enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_device_48m_clk_en(
        &mut self,
    ) -> LpAonclkrstUsbDevice48mClkEnW<'_, LpAonclkrstHpUsbClkrstCtrl0Spec> {
        LpAonclkrstUsbDevice48mClkEnW::new(self, 5)
    }
    #[doc = "Bits 6:13 - usb 480m to 25m divide number."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_48m_div_num(
        &mut self,
    ) -> LpAonclkrstUsb48mDivNumW<'_, LpAonclkrstHpUsbClkrstCtrl0Spec> {
        LpAonclkrstUsb48mDivNumW::new(self, 6)
    }
    #[doc = "Bits 14:21 - usb 500m to 25m divide number."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_25m_div_num(
        &mut self,
    ) -> LpAonclkrstUsb25mDivNumW<'_, LpAonclkrstHpUsbClkrstCtrl0Spec> {
        LpAonclkrstUsb25mDivNumW::new(self, 14)
    }
    #[doc = "Bits 22:29 - usb 480m to 12m divide number."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_12m_div_num(
        &mut self,
    ) -> LpAonclkrstUsb12mDivNumW<'_, LpAonclkrstHpUsbClkrstCtrl0Spec> {
        LpAonclkrstUsb12mDivNumW::new(self, 22)
    }
}
#[doc = "HP USB Clock Reset Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hp_usb_clkrst_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_usb_clkrst_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpAonclkrstHpUsbClkrstCtrl0Spec;
impl crate::RegisterSpec for LpAonclkrstHpUsbClkrstCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hp_usb_clkrst_ctrl0::R`](R) reader structure"]
impl crate::Readable for LpAonclkrstHpUsbClkrstCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_hp_usb_clkrst_ctrl0::W`](W) writer structure"]
impl crate::Writable for LpAonclkrstHpUsbClkrstCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HP_USB_CLKRST_CTRL0 to value 0x09c4_c27a"]
impl crate::Resettable for LpAonclkrstHpUsbClkrstCtrl0Spec {
    const RESET_VALUE: u32 = 0x09c4_c27a;
}
