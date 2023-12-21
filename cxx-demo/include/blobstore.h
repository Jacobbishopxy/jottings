/**
 * @file:	blobstore.h
 * @author:	Jacob Xie
 * @date:	2023/12/21 16:42:38 Thursday
 * @brief:
 **/

#pragma once
#include <memory>

struct MultiBuf;

class BlobstoreClient
{
public:
  BlobstoreClient();

  uint64_t put(MultiBuf& buf) const;
};

std::unique_ptr<BlobstoreClient> new_blobstore_client();
