#[doc = "Register `HP_POR_RST_BYPASS_CTRL` reader"]
pub type R = crate::R<HpPorRstBypassCtrlSpec>;
#[doc = "Register `HP_POR_RST_BYPASS_CTRL` writer"]
pub type W = crate::W<HpPorRstBypassCtrlSpec>;
#[doc = "Field `HP_PO_CNNT_RSTN_BYPASS_CTRL` reader - 15\\] 1'b1: po_cnnt_rstn bypass sys_sw_rstn \\[14\\] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn \\[13\\] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn \\[12\\] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn \\[11\\] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst \\[10\\] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst \\[9\\] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn \\[8\\] 1'b1: po_cnnt_rstn bypass efuse_err_rstn"]
pub type HpPoCnntRstnBypassCtrlR = crate::FieldReader;
#[doc = "Field `HP_PO_CNNT_RSTN_BYPASS_CTRL` writer - 15\\] 1'b1: po_cnnt_rstn bypass sys_sw_rstn \\[14\\] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn \\[13\\] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn \\[12\\] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn \\[11\\] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst \\[10\\] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst \\[9\\] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn \\[8\\] 1'b1: po_cnnt_rstn bypass efuse_err_rstn"]
pub type HpPoCnntRstnBypassCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HP_PO_RSTN_BYPASS_CTRL` reader - 31\\] 1'b1: po_rstn bypass sys_sw_rstn \\[30\\] 1'b1: po_rstn bypass hp_wdt_sys_rstn \\[29\\] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn \\[28\\] 1'b1: po_rstn bypass hp_sdio_sys_rstn \\[27\\] 1'b1: po_rstn bypass usb_jtag_chip_rst \\[26\\] 1'b1: po_rstn bypass usb_uart_chip_rst \\[25\\] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn \\[24\\] 1'b1: po_rstn bypass efuse_err_rstn"]
pub type HpPoRstnBypassCtrlR = crate::FieldReader;
#[doc = "Field `HP_PO_RSTN_BYPASS_CTRL` writer - 31\\] 1'b1: po_rstn bypass sys_sw_rstn \\[30\\] 1'b1: po_rstn bypass hp_wdt_sys_rstn \\[29\\] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn \\[28\\] 1'b1: po_rstn bypass hp_sdio_sys_rstn \\[27\\] 1'b1: po_rstn bypass usb_jtag_chip_rst \\[26\\] 1'b1: po_rstn bypass usb_uart_chip_rst \\[25\\] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn \\[24\\] 1'b1: po_rstn bypass efuse_err_rstn"]
pub type HpPoRstnBypassCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15\\] 1'b1: po_cnnt_rstn bypass sys_sw_rstn \\[14\\] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn \\[13\\] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn \\[12\\] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn \\[11\\] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst \\[10\\] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst \\[9\\] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn \\[8\\] 1'b1: po_cnnt_rstn bypass efuse_err_rstn"]
    #[inline(always)]
    pub fn hp_po_cnnt_rstn_bypass_ctrl(&self) -> HpPoCnntRstnBypassCtrlR {
        HpPoCnntRstnBypassCtrlR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31\\] 1'b1: po_rstn bypass sys_sw_rstn \\[30\\] 1'b1: po_rstn bypass hp_wdt_sys_rstn \\[29\\] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn \\[28\\] 1'b1: po_rstn bypass hp_sdio_sys_rstn \\[27\\] 1'b1: po_rstn bypass usb_jtag_chip_rst \\[26\\] 1'b1: po_rstn bypass usb_uart_chip_rst \\[25\\] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn \\[24\\] 1'b1: po_rstn bypass efuse_err_rstn"]
    #[inline(always)]
    pub fn hp_po_rstn_bypass_ctrl(&self) -> HpPoRstnBypassCtrlR {
        HpPoRstnBypassCtrlR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15\\] 1'b1: po_cnnt_rstn bypass sys_sw_rstn \\[14\\] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn \\[13\\] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn \\[12\\] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn \\[11\\] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst \\[10\\] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst \\[9\\] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn \\[8\\] 1'b1: po_cnnt_rstn bypass efuse_err_rstn"]
    #[inline(always)]
    pub fn hp_po_cnnt_rstn_bypass_ctrl(
        &mut self,
    ) -> HpPoCnntRstnBypassCtrlW<'_, HpPorRstBypassCtrlSpec> {
        HpPoCnntRstnBypassCtrlW::new(self, 8)
    }
    #[doc = "Bits 24:31 - 31\\] 1'b1: po_rstn bypass sys_sw_rstn \\[30\\] 1'b1: po_rstn bypass hp_wdt_sys_rstn \\[29\\] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn \\[28\\] 1'b1: po_rstn bypass hp_sdio_sys_rstn \\[27\\] 1'b1: po_rstn bypass usb_jtag_chip_rst \\[26\\] 1'b1: po_rstn bypass usb_uart_chip_rst \\[25\\] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn \\[24\\] 1'b1: po_rstn bypass efuse_err_rstn"]
    #[inline(always)]
    pub fn hp_po_rstn_bypass_ctrl(&mut self) -> HpPoRstnBypassCtrlW<'_, HpPorRstBypassCtrlSpec> {
        HpPoRstnBypassCtrlW::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_por_rst_bypass_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_por_rst_bypass_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpPorRstBypassCtrlSpec;
impl crate::RegisterSpec for HpPorRstBypassCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_por_rst_bypass_ctrl::R`](R) reader structure"]
impl crate::Readable for HpPorRstBypassCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hp_por_rst_bypass_ctrl::W`](W) writer structure"]
impl crate::Writable for HpPorRstBypassCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_POR_RST_BYPASS_CTRL to value 0xff00_ff00"]
impl crate::Resettable for HpPorRstBypassCtrlSpec {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
