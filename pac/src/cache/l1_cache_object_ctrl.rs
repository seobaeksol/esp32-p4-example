#[doc = "Register `L1_CACHE_OBJECT_CTRL` reader"]
pub type R = crate::R<L1CacheObjectCtrlSpec>;
#[doc = "Register `L1_CACHE_OBJECT_CTRL` writer"]
pub type W = crate::W<L1CacheObjectCtrlSpec>;
#[doc = "Field `L1_ICACHE0_TAG_OBJECT` reader - Set this bit to set L1-ICache0 tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1Icache0TagObjectR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_TAG_OBJECT` writer - Set this bit to set L1-ICache0 tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1Icache0TagObjectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_TAG_OBJECT` reader - Set this bit to set L1-ICache1 tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1Icache1TagObjectR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_TAG_OBJECT` writer - Set this bit to set L1-ICache1 tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1Icache1TagObjectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_TAG_OBJECT` reader - Reserved"]
pub type L1Icache2TagObjectR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_TAG_OBJECT` reader - Reserved"]
pub type L1Icache3TagObjectR = crate::BitReader;
#[doc = "Field `L1_DCACHE_TAG_OBJECT` reader - Set this bit to set L1-DCache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1DcacheTagObjectR = crate::BitReader;
#[doc = "Field `L1_DCACHE_TAG_OBJECT` writer - Set this bit to set L1-DCache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1DcacheTagObjectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_MEM_OBJECT` reader - Set this bit to set L1-ICache0 data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1Icache0MemObjectR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_MEM_OBJECT` writer - Set this bit to set L1-ICache0 data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1Icache0MemObjectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_MEM_OBJECT` reader - Set this bit to set L1-ICache1 data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1Icache1MemObjectR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_MEM_OBJECT` writer - Set this bit to set L1-ICache1 data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1Icache1MemObjectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_MEM_OBJECT` reader - Reserved"]
pub type L1Icache2MemObjectR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_MEM_OBJECT` reader - Reserved"]
pub type L1Icache3MemObjectR = crate::BitReader;
#[doc = "Field `L1_DCACHE_MEM_OBJECT` reader - Set this bit to set L1-DCache data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1DcacheMemObjectR = crate::BitReader;
#[doc = "Field `L1_DCACHE_MEM_OBJECT` writer - Set this bit to set L1-DCache data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L1DcacheMemObjectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to set L1-ICache0 tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache0_tag_object(&self) -> L1Icache0TagObjectR {
        L1Icache0TagObjectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to set L1-ICache1 tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache1_tag_object(&self) -> L1Icache1TagObjectR {
        L1Icache1TagObjectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_tag_object(&self) -> L1Icache2TagObjectR {
        L1Icache2TagObjectR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_tag_object(&self) -> L1Icache3TagObjectR {
        L1Icache3TagObjectR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to set L1-DCache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_dcache_tag_object(&self) -> L1DcacheTagObjectR {
        L1DcacheTagObjectR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to set L1-ICache0 data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache0_mem_object(&self) -> L1Icache0MemObjectR {
        L1Icache0MemObjectR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to set L1-ICache1 data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache1_mem_object(&self) -> L1Icache1MemObjectR {
        L1Icache1MemObjectR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_mem_object(&self) -> L1Icache2MemObjectR {
        L1Icache2MemObjectR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_mem_object(&self) -> L1Icache3MemObjectR {
        L1Icache3MemObjectR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to set L1-DCache data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_dcache_mem_object(&self) -> L1DcacheMemObjectR {
        L1DcacheMemObjectR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to set L1-ICache0 tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache0_tag_object(&mut self) -> L1Icache0TagObjectW<'_, L1CacheObjectCtrlSpec> {
        L1Icache0TagObjectW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to set L1-ICache1 tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache1_tag_object(&mut self) -> L1Icache1TagObjectW<'_, L1CacheObjectCtrlSpec> {
        L1Icache1TagObjectW::new(self, 1)
    }
    #[doc = "Bit 4 - Set this bit to set L1-DCache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_dcache_tag_object(&mut self) -> L1DcacheTagObjectW<'_, L1CacheObjectCtrlSpec> {
        L1DcacheTagObjectW::new(self, 4)
    }
    #[doc = "Bit 6 - Set this bit to set L1-ICache0 data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache0_mem_object(&mut self) -> L1Icache0MemObjectW<'_, L1CacheObjectCtrlSpec> {
        L1Icache0MemObjectW::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to set L1-ICache1 data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_icache1_mem_object(&mut self) -> L1Icache1MemObjectW<'_, L1CacheObjectCtrlSpec> {
        L1Icache1MemObjectW::new(self, 7)
    }
    #[doc = "Bit 10 - Set this bit to set L1-DCache data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l1_dcache_mem_object(&mut self) -> L1DcacheMemObjectW<'_, L1CacheObjectCtrlSpec> {
        L1DcacheMemObjectW::new(self, 10)
    }
}
#[doc = "Cache Tag and Data memory Object control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_object_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_object_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheObjectCtrlSpec;
impl crate::RegisterSpec for L1CacheObjectCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_object_ctrl::R`](R) reader structure"]
impl crate::Readable for L1CacheObjectCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_object_ctrl::W`](W) writer structure"]
impl crate::Writable for L1CacheObjectCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_OBJECT_CTRL to value 0"]
impl crate::Resettable for L1CacheObjectCtrlSpec {}
