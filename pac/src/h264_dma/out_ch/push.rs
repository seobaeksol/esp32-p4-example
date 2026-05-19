#[doc = "Register `PUSH` reader"]
pub type R = crate::R<PushSpec>;
#[doc = "Register `PUSH` writer"]
pub type W = crate::W<PushSpec>;
#[doc = "Field `OUTFIFO_WDATA_CH0` reader - This register stores the data that need to be pushed into DMA Tx FIFO."]
pub type OutfifoWdataCh0R = crate::FieldReader<u16>;
#[doc = "Field `OUTFIFO_WDATA_CH0` writer - This register stores the data that need to be pushed into DMA Tx FIFO."]
pub type OutfifoWdataCh0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OUTFIFO_PUSH_CH0` reader - Set this bit to push data into DMA Tx FIFO."]
pub type OutfifoPushCh0R = crate::BitReader;
#[doc = "Field `OUTFIFO_PUSH_CH0` writer - Set this bit to push data into DMA Tx FIFO."]
pub type OutfifoPushCh0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - This register stores the data that need to be pushed into DMA Tx FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata_ch0(&self) -> OutfifoWdataCh0R {
        OutfifoWdataCh0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Set this bit to push data into DMA Tx FIFO."]
    #[inline(always)]
    pub fn outfifo_push_ch0(&self) -> OutfifoPushCh0R {
        OutfifoPushCh0R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register stores the data that need to be pushed into DMA Tx FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata_ch0(&mut self) -> OutfifoWdataCh0W<'_, PushSpec> {
        OutfifoWdataCh0W::new(self, 0)
    }
    #[doc = "Bit 10 - Set this bit to push data into DMA Tx FIFO."]
    #[inline(always)]
    pub fn outfifo_push_ch0(&mut self) -> OutfifoPushCh0W<'_, PushSpec> {
        OutfifoPushCh0W::new(self, 10)
    }
}
#[doc = "TX CHx outfifo push register\n\nYou can [`read`](crate::Reg::read) this register and get [`push::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`push::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PushSpec;
impl crate::RegisterSpec for PushSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`push::R`](R) reader structure"]
impl crate::Readable for PushSpec {}
#[doc = "`write(|w| ..)` method takes [`push::W`](W) writer structure"]
impl crate::Writable for PushSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUSH to value 0"]
impl crate::Resettable for PushSpec {}
