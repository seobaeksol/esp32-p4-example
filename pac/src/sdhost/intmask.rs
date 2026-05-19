#[doc = "Register `INTMASK` reader"]
pub type R = crate::R<IntmaskSpec>;
#[doc = "Register `INTMASK` writer"]
pub type W = crate::W<IntmaskSpec>;
#[doc = "Field `INT_MASK` reader - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
pub type IntMaskR = crate::FieldReader<u16>;
#[doc = "Field `INT_MASK` writer - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
pub type IntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SDIO_INT_MASK` reader - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
pub type SdioIntMaskR = crate::FieldReader;
#[doc = "Field `SDIO_INT_MASK` writer - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
pub type SdioIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
    #[inline(always)]
    pub fn int_mask(&self) -> IntMaskR {
        IntMaskR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> SdioIntMaskR {
        SdioIntMaskR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - These bits used to mask unwanted interrupts. A value of 0 masks interrupt, and a value of 1 enables the interrupt. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): Rx Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation-by-host timeout; Bit 9 (DRTO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect."]
    #[inline(always)]
    pub fn int_mask(&mut self) -> IntMaskW<'_, IntmaskSpec> {
        IntMaskW::new(self, 0)
    }
    #[doc = "Bits 16:17 - SDIO interrupt mask, one bit for each card. Bit\\[17:16\\] correspond to card\\[15:0\\] respectively. When masked, SDIO interrupt detection for that card is disabled. 0 masks an interrupt, and 1 enables an interrupt."]
    #[inline(always)]
    pub fn sdio_int_mask(&mut self) -> SdioIntMaskW<'_, IntmaskSpec> {
        SdioIntMaskW::new(self, 16)
    }
}
#[doc = "SDIO interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntmaskSpec;
impl crate::RegisterSpec for IntmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmask::R`](R) reader structure"]
impl crate::Readable for IntmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intmask::W`](W) writer structure"]
impl crate::Writable for IntmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for IntmaskSpec {}
