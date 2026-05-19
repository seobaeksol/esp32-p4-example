#[doc = "Register `L1_DCACHE_BLOCKSIZE_CONF` reader"]
pub type R = crate::R<L1DcacheBlocksizeConfSpec>;
#[doc = "Field `L1_DCACHE_BLOCKSIZE_8` reader - The field is used to configureblocksize of L1-DCache as 8 bytes. This field and all other fields within this register is onehot."]
pub type L1DcacheBlocksize8R = crate::BitReader;
#[doc = "Field `L1_DCACHE_BLOCKSIZE_16` reader - The field is used to configureblocksize of L1-DCache as 16 bytes. This field and all other fields within this register is onehot."]
pub type L1DcacheBlocksize16R = crate::BitReader;
#[doc = "Field `L1_DCACHE_BLOCKSIZE_32` reader - The field is used to configureblocksize of L1-DCache as 32 bytes. This field and all other fields within this register is onehot."]
pub type L1DcacheBlocksize32R = crate::BitReader;
#[doc = "Field `L1_DCACHE_BLOCKSIZE_64` reader - The field is used to configureblocksize of L1-DCache as 64 bytes. This field and all other fields within this register is onehot."]
pub type L1DcacheBlocksize64R = crate::BitReader;
#[doc = "Field `L1_DCACHE_BLOCKSIZE_128` reader - The field is used to configureblocksize of L1-DCache as 128 bytes. This field and all other fields within this register is onehot."]
pub type L1DcacheBlocksize128R = crate::BitReader;
#[doc = "Field `L1_DCACHE_BLOCKSIZE_256` reader - The field is used to configureblocksize of L1-DCache as 256 bytes. This field and all other fields within this register is onehot."]
pub type L1DcacheBlocksize256R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The field is used to configureblocksize of L1-DCache as 8 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_dcache_blocksize_8(&self) -> L1DcacheBlocksize8R {
        L1DcacheBlocksize8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The field is used to configureblocksize of L1-DCache as 16 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_dcache_blocksize_16(&self) -> L1DcacheBlocksize16R {
        L1DcacheBlocksize16R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The field is used to configureblocksize of L1-DCache as 32 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_dcache_blocksize_32(&self) -> L1DcacheBlocksize32R {
        L1DcacheBlocksize32R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The field is used to configureblocksize of L1-DCache as 64 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_dcache_blocksize_64(&self) -> L1DcacheBlocksize64R {
        L1DcacheBlocksize64R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The field is used to configureblocksize of L1-DCache as 128 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_dcache_blocksize_128(&self) -> L1DcacheBlocksize128R {
        L1DcacheBlocksize128R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The field is used to configureblocksize of L1-DCache as 256 bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn l1_dcache_blocksize_256(&self) -> L1DcacheBlocksize256R {
        L1DcacheBlocksize256R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "L1 data Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_blocksize_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DcacheBlocksizeConfSpec;
impl crate::RegisterSpec for L1DcacheBlocksizeConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_blocksize_conf::R`](R) reader structure"]
impl crate::Readable for L1DcacheBlocksizeConfSpec {}
#[doc = "`reset()` method sets L1_DCACHE_BLOCKSIZE_CONF to value 0x08"]
impl crate::Resettable for L1DcacheBlocksizeConfSpec {
    const RESET_VALUE: u32 = 0x08;
}
