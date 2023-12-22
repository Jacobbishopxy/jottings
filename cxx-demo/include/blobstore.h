/**
 * @file:	blobstore.h
 * @author:	Jacob Xie
 * @date:	2023/12/21 16:42:38 Thursday
 * @brief:
 **/

#pragma once

#include "rust/cxx.h"
#include <memory>

struct MultiBuf;
struct BlobMetadata;

class BlobstoreClient
{
public:
  BlobstoreClient();
  uint64_t put(MultiBuf& buf) const;
  void tag(uint64_t blobid, rust::Str tag) const;
  BlobMetadata metadata(uint64_t blobid) const;

private:
  class impl;
  std::shared_ptr<impl> impl;
};

std::unique_ptr<BlobstoreClient> new_blobstore_client();
