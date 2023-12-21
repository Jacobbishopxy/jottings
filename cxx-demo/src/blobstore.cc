/**
 * @file:	blobstore.cc
 * @author:	Jacob Xie
 * @date:	2023/12/21 16:58:24 Thursday
 * @brief:
 **/

#include "cxx-demo/include/blobstore.h"
#include "cxx-demo/src/main.rs.h"
#include <functional>
#include <string>

BlobstoreClient::BlobstoreClient() {}

std::unique_ptr<BlobstoreClient> new_blobstore_client()
{
  return std::unique_ptr<BlobstoreClient>(new BlobstoreClient());
}

// Upload a new blob and return a blobid that serves as a handle to the blob.
uint64_t BlobstoreClient::put(MultiBuf& buf) const
{
  // Traverse the caller's chunk iterator.
  std::string contents;

  while (true)
  {
    auto chunk = next_chunk(buf);
    if (chunk.size() == 0)
    {
      break;
    }
    contents.append(reinterpret_cast<const char*>(chunk.data()), chunk.size());
  }

  // Pretend we did something useful to persist the data.
  auto blobid = std::hash<std::string>{}(contents);
  return blobid;
}
