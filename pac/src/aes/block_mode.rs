#[doc = "Register `BLOCK_MODE` reader"]
pub type R = crate::R<BlockModeSpec>;
#[doc = "Register `BLOCK_MODE` writer"]
pub type W = crate::W<BlockModeSpec>;
#[doc = "Field `BLOCK_MODE` reader - Configures the block cipher mode of the AES accelerator operating under the DMA-AES working mode. \\\\ 0: ECB (Electronic Code Block)\\\\ 1: CBC (Cipher Block Chaining)\\\\ 2: OFB (Output FeedBack)\\\\ 3: CTR (Counter)\\\\ 4: CFB8 (8-bit Cipher FeedBack)\\\\ 5: CFB128 (128-bit Cipher FeedBack)\\\\ 6: GCM\\\\ 7: Reserved\\\\"]
pub type BlockModeR = crate::FieldReader;
#[doc = "Field `BLOCK_MODE` writer - Configures the block cipher mode of the AES accelerator operating under the DMA-AES working mode. \\\\ 0: ECB (Electronic Code Block)\\\\ 1: CBC (Cipher Block Chaining)\\\\ 2: OFB (Output FeedBack)\\\\ 3: CTR (Counter)\\\\ 4: CFB8 (8-bit Cipher FeedBack)\\\\ 5: CFB128 (128-bit Cipher FeedBack)\\\\ 6: GCM\\\\ 7: Reserved\\\\"]
pub type BlockModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Configures the block cipher mode of the AES accelerator operating under the DMA-AES working mode. \\\\ 0: ECB (Electronic Code Block)\\\\ 1: CBC (Cipher Block Chaining)\\\\ 2: OFB (Output FeedBack)\\\\ 3: CTR (Counter)\\\\ 4: CFB8 (8-bit Cipher FeedBack)\\\\ 5: CFB128 (128-bit Cipher FeedBack)\\\\ 6: GCM\\\\ 7: Reserved\\\\"]
    #[inline(always)]
    pub fn block_mode(&self) -> BlockModeR {
        BlockModeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures the block cipher mode of the AES accelerator operating under the DMA-AES working mode. \\\\ 0: ECB (Electronic Code Block)\\\\ 1: CBC (Cipher Block Chaining)\\\\ 2: OFB (Output FeedBack)\\\\ 3: CTR (Counter)\\\\ 4: CFB8 (8-bit Cipher FeedBack)\\\\ 5: CFB128 (128-bit Cipher FeedBack)\\\\ 6: GCM\\\\ 7: Reserved\\\\"]
    #[inline(always)]
    pub fn block_mode(&mut self) -> BlockModeW<'_, BlockModeSpec> {
        BlockModeW::new(self, 0)
    }
}
#[doc = "Defines the block cipher mode\n\nYou can [`read`](crate::Reg::read) this register and get [`block_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlockModeSpec;
impl crate::RegisterSpec for BlockModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`block_mode::R`](R) reader structure"]
impl crate::Readable for BlockModeSpec {}
#[doc = "`write(|w| ..)` method takes [`block_mode::W`](W) writer structure"]
impl crate::Writable for BlockModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLOCK_MODE to value 0"]
impl crate::Resettable for BlockModeSpec {}
