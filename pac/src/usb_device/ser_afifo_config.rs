#[doc = "Register `SER_AFIFO_CONFIG` reader"]
pub type R = crate::R<SerAfifoConfigSpec>;
#[doc = "Register `SER_AFIFO_CONFIG` writer"]
pub type W = crate::W<SerAfifoConfigSpec>;
#[doc = "Field `SERIAL_IN_AFIFO_RESET_WR` reader - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
pub type SerialInAfifoResetWrR = crate::BitReader;
#[doc = "Field `SERIAL_IN_AFIFO_RESET_WR` writer - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
pub type SerialInAfifoResetWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_IN_AFIFO_RESET_RD` reader - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
pub type SerialInAfifoResetRdR = crate::BitReader;
#[doc = "Field `SERIAL_IN_AFIFO_RESET_RD` writer - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
pub type SerialInAfifoResetRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_OUT_AFIFO_RESET_WR` reader - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
pub type SerialOutAfifoResetWrR = crate::BitReader;
#[doc = "Field `SERIAL_OUT_AFIFO_RESET_WR` writer - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
pub type SerialOutAfifoResetWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_OUT_AFIFO_RESET_RD` reader - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
pub type SerialOutAfifoResetRdR = crate::BitReader;
#[doc = "Field `SERIAL_OUT_AFIFO_RESET_RD` writer - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
pub type SerialOutAfifoResetRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_OUT_AFIFO_REMPTY` reader - CDC_ACM OUTOUT async FIFO empty signal in read clock domain."]
pub type SerialOutAfifoRemptyR = crate::BitReader;
#[doc = "Field `SERIAL_IN_AFIFO_WFULL` reader - CDC_ACM OUT IN async FIFO empty signal in write clock domain."]
pub type SerialInAfifoWfullR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
    #[inline(always)]
    pub fn serial_in_afifo_reset_wr(&self) -> SerialInAfifoResetWrR {
        SerialInAfifoResetWrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
    #[inline(always)]
    pub fn serial_in_afifo_reset_rd(&self) -> SerialInAfifoResetRdR {
        SerialInAfifoResetRdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
    #[inline(always)]
    pub fn serial_out_afifo_reset_wr(&self) -> SerialOutAfifoResetWrR {
        SerialOutAfifoResetWrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
    #[inline(always)]
    pub fn serial_out_afifo_reset_rd(&self) -> SerialOutAfifoResetRdR {
        SerialOutAfifoResetRdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CDC_ACM OUTOUT async FIFO empty signal in read clock domain."]
    #[inline(always)]
    pub fn serial_out_afifo_rempty(&self) -> SerialOutAfifoRemptyR {
        SerialOutAfifoRemptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CDC_ACM OUT IN async FIFO empty signal in write clock domain."]
    #[inline(always)]
    pub fn serial_in_afifo_wfull(&self) -> SerialInAfifoWfullR {
        SerialInAfifoWfullR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
    #[inline(always)]
    pub fn serial_in_afifo_reset_wr(&mut self) -> SerialInAfifoResetWrW<'_, SerAfifoConfigSpec> {
        SerialInAfifoResetWrW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
    #[inline(always)]
    pub fn serial_in_afifo_reset_rd(&mut self) -> SerialInAfifoResetRdW<'_, SerAfifoConfigSpec> {
        SerialInAfifoResetRdW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
    #[inline(always)]
    pub fn serial_out_afifo_reset_wr(&mut self) -> SerialOutAfifoResetWrW<'_, SerAfifoConfigSpec> {
        SerialOutAfifoResetWrW::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
    #[inline(always)]
    pub fn serial_out_afifo_reset_rd(&mut self) -> SerialOutAfifoResetRdW<'_, SerAfifoConfigSpec> {
        SerialOutAfifoResetRdW::new(self, 3)
    }
}
#[doc = "Serial AFIFO configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`ser_afifo_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ser_afifo_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SerAfifoConfigSpec;
impl crate::RegisterSpec for SerAfifoConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ser_afifo_config::R`](R) reader structure"]
impl crate::Readable for SerAfifoConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`ser_afifo_config::W`](W) writer structure"]
impl crate::Writable for SerAfifoConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SER_AFIFO_CONFIG to value 0x10"]
impl crate::Resettable for SerAfifoConfigSpec {
    const RESET_VALUE: u32 = 0x10;
}
