#[doc = "Register `L1_ICACHE_BLOCKSIZE_CONF` reader"]
pub type R = crate::R<L1IcacheBlocksizeConfSpec>;
#[doc = "Field `L1_ICACHE_BLOCKSIZE_8` reader - The field is used to configureblocksize of L1-ICache as 8 bytes. This field and all other fields within this register is onehot."]
pub type L1IcacheBlocksize8R = crate::BitReader;
#[doc = "Field `L1_ICACHE_BLOCKSIZE_16` reader - The field is used to configureblocksize of L1-ICache as 16 bytes. This field and all other fields within this register is onehot."]
pub type L1IcacheBlocksize16R = crate::BitReader;
#[doc = "Field `L1_ICACHE_BLOCKSIZE_32` reader - The field is used to configureblocksize of L1-ICache as 32 bytes. This field and all other fields within this register is onehot."]
pub type L1IcacheBlocksize32R = crate::BitReader;
#[doc = "Field `L1_ICACHE_BLOCKSIZE_64` reader - The field is used to configureblocksize of L1-ICache as 64 bytes. This field and all other fields within this register is onehot."]
pub type L1IcacheBlocksize64R = crate::BitReader;
#[doc = "Field `L1_ICACHE_BLOCKSIZE_128` reader - The field is used to configureblocksize of L1-ICache as 128 bytes. This field and all other fields within this register is onehot."]
pub type L1IcacheBlocksize128R = crate::BitReader;
#[doc = "Field `L1_ICACHE_BLOCKSIZE_256` reader - The field is used to configureblocksize of L1-ICache as 256 bytes. This field and all other fields within this register is onehot."]
pub type L1IcacheBlocksize256R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The field is used to configureblocksize of L1-ICache as 8 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_icache_blocksize_8(&self) -> L1IcacheBlocksize8R {
        L1IcacheBlocksize8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The field is used to configureblocksize of L1-ICache as 16 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_icache_blocksize_16(&self) -> L1IcacheBlocksize16R {
        L1IcacheBlocksize16R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The field is used to configureblocksize of L1-ICache as 32 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_icache_blocksize_32(&self) -> L1IcacheBlocksize32R {
        L1IcacheBlocksize32R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The field is used to configureblocksize of L1-ICache as 64 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_icache_blocksize_64(&self) -> L1IcacheBlocksize64R {
        L1IcacheBlocksize64R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The field is used to configureblocksize of L1-ICache as 128 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_icache_blocksize_128(&self) -> L1IcacheBlocksize128R {
        L1IcacheBlocksize128R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The field is used to configureblocksize of L1-ICache as 256 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_icache_blocksize_256(&self) -> L1IcacheBlocksize256R {
        L1IcacheBlocksize256R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "L1 instruction Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache_blocksize_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1IcacheBlocksizeConfSpec;
impl crate::RegisterSpec for L1IcacheBlocksizeConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache_blocksize_conf::R`](R) reader structure"]
impl crate::Readable for L1IcacheBlocksizeConfSpec {}
#[doc = "`reset()` method sets L1_ICACHE_BLOCKSIZE_CONF to value 0x08"]
impl crate::Resettable for L1IcacheBlocksizeConfSpec {
    const RESET_VALUE: u32 = 0x08;
}
