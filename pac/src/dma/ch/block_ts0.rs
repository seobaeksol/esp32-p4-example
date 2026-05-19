#[doc = "Register `BLOCK_TS0` reader"]
pub type R = crate::R<BlockTs0Spec>;
#[doc = "Register `BLOCK_TS0` writer"]
pub type W = crate::W<BlockTs0Spec>;
#[doc = "Field `CH1_BLOCK_TS` reader - NA"]
pub type Ch1BlockTsR = crate::FieldReader<u32>;
#[doc = "Field `CH1_BLOCK_TS` writer - NA"]
pub type Ch1BlockTsW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - NA"]
    #[inline(always)]
    pub fn ch1_block_ts(&self) -> Ch1BlockTsR {
        Ch1BlockTsR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - NA"]
    #[inline(always)]
    pub fn ch1_block_ts(&mut self) -> Ch1BlockTsW<'_, BlockTs0Spec> {
        Ch1BlockTsW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`block_ts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_ts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlockTs0Spec;
impl crate::RegisterSpec for BlockTs0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`block_ts0::R`](R) reader structure"]
impl crate::Readable for BlockTs0Spec {}
#[doc = "`write(|w| ..)` method takes [`block_ts0::W`](W) writer structure"]
impl crate::Writable for BlockTs0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLOCK_TS0 to value 0"]
impl crate::Resettable for BlockTs0Spec {}
