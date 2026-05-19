#[doc = "Register `CONF1` reader"]
pub type R = crate::R<Conf1Spec>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<Conf1Spec>;
#[doc = "Field `RXFIFO_FULL_THRHD` reader - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
pub type RxfifoFullThrhdR = crate::FieldReader;
#[doc = "Field `RXFIFO_FULL_THRHD` writer - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
pub type RxfifoFullThrhdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TXFIFO_EMPTY_THRHD` reader - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
pub type TxfifoEmptyThrhdR = crate::FieldReader;
#[doc = "Field `TXFIFO_EMPTY_THRHD` writer - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
pub type TxfifoEmptyThrhdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CTS_INV` reader - Set this bit to inverse the level value of uart cts signal."]
pub type CtsInvR = crate::BitReader;
#[doc = "Field `CTS_INV` writer - Set this bit to inverse the level value of uart cts signal."]
pub type CtsInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR_INV` reader - Set this bit to inverse the level value of uart dsr signal."]
pub type DsrInvR = crate::BitReader;
#[doc = "Field `DSR_INV` writer - Set this bit to inverse the level value of uart dsr signal."]
pub type DsrInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_INV` reader - Set this bit to inverse the level value of uart rts signal."]
pub type RtsInvR = crate::BitReader;
#[doc = "Field `RTS_INV` writer - Set this bit to inverse the level value of uart rts signal."]
pub type RtsInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR_INV` reader - Set this bit to inverse the level value of uart dtr signal."]
pub type DtrInvR = crate::BitReader;
#[doc = "Field `DTR_INV` writer - Set this bit to inverse the level value of uart dtr signal."]
pub type DtrInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DTR` reader - This register is used to configure the software dtr signal which is used in software flow control."]
pub type SwDtrR = crate::BitReader;
#[doc = "Field `SW_DTR` writer - This register is used to configure the software dtr signal which is used in software flow control."]
pub type SwDtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 3:7 - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RxfifoFullThrhdR {
        RxfifoFullThrhdR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TxfifoEmptyThrhdR {
        TxfifoEmptyThrhdR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Set this bit to inverse the level value of uart cts signal."]
    #[inline(always)]
    pub fn cts_inv(&self) -> CtsInvR {
        CtsInvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to inverse the level value of uart dsr signal."]
    #[inline(always)]
    pub fn dsr_inv(&self) -> DsrInvR {
        DsrInvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to inverse the level value of uart rts signal."]
    #[inline(always)]
    pub fn rts_inv(&self) -> RtsInvR {
        RtsInvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to inverse the level value of uart dtr signal."]
    #[inline(always)]
    pub fn dtr_inv(&self) -> DtrInvR {
        DtrInvR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This register is used to configure the software dtr signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_dtr(&self) -> SwDtrR {
        SwDtrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:7 - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&mut self) -> RxfifoFullThrhdW<'_, Conf1Spec> {
        RxfifoFullThrhdW::new(self, 3)
    }
    #[doc = "Bits 11:15 - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&mut self) -> TxfifoEmptyThrhdW<'_, Conf1Spec> {
        TxfifoEmptyThrhdW::new(self, 11)
    }
    #[doc = "Bit 16 - Set this bit to inverse the level value of uart cts signal."]
    #[inline(always)]
    pub fn cts_inv(&mut self) -> CtsInvW<'_, Conf1Spec> {
        CtsInvW::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to inverse the level value of uart dsr signal."]
    #[inline(always)]
    pub fn dsr_inv(&mut self) -> DsrInvW<'_, Conf1Spec> {
        DsrInvW::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to inverse the level value of uart rts signal."]
    #[inline(always)]
    pub fn rts_inv(&mut self) -> RtsInvW<'_, Conf1Spec> {
        RtsInvW::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to inverse the level value of uart dtr signal."]
    #[inline(always)]
    pub fn dtr_inv(&mut self) -> DtrInvW<'_, Conf1Spec> {
        DtrInvW::new(self, 19)
    }
    #[doc = "Bit 20 - This register is used to configure the software dtr signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_dtr(&mut self) -> SwDtrW<'_, Conf1Spec> {
        SwDtrW::new(self, 20)
    }
    #[doc = "Bit 21 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, Conf1Spec> {
        ClkEnW::new(self, 21)
    }
}
#[doc = "Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Conf1Spec;
impl crate::RegisterSpec for Conf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for Conf1Spec {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for Conf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF1 to value 0x6060"]
impl crate::Resettable for Conf1Spec {
    const RESET_VALUE: u32 = 0x6060;
}
