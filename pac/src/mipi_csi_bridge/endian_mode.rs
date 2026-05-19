#[doc = "Register `ENDIAN_MODE` reader"]
pub type R = crate::R<EndianModeSpec>;
#[doc = "Register `ENDIAN_MODE` writer"]
pub type W = crate::W<EndianModeSpec>;
#[doc = "Field `BYTE_ENDIAN_ORDER` reader - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed."]
pub type ByteEndianOrderR = crate::BitReader;
#[doc = "Field `BYTE_ENDIAN_ORDER` writer - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed."]
pub type ByteEndianOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ENDIAN_ORDER` reader - N/A"]
pub type BitEndianOrderR = crate::BitReader;
#[doc = "Field `BIT_ENDIAN_ORDER` writer - N/A"]
pub type BitEndianOrderW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed."]
    #[inline(always)]
    pub fn byte_endian_order(&self) -> ByteEndianOrderR {
        ByteEndianOrderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn bit_endian_order(&self) -> BitEndianOrderR {
        BitEndianOrderR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed."]
    #[inline(always)]
    pub fn byte_endian_order(&mut self) -> ByteEndianOrderW<'_, EndianModeSpec> {
        ByteEndianOrderW::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn bit_endian_order(&mut self) -> BitEndianOrderW<'_, EndianModeSpec> {
        BitEndianOrderW::new(self, 1)
    }
}
#[doc = "data endianness order configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`endian_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endian_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndianModeSpec;
impl crate::RegisterSpec for EndianModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endian_mode::R`](R) reader structure"]
impl crate::Readable for EndianModeSpec {}
#[doc = "`write(|w| ..)` method takes [`endian_mode::W`](W) writer structure"]
impl crate::Writable for EndianModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENDIAN_MODE to value 0"]
impl crate::Resettable for EndianModeSpec {}
