/**
 * @file:	blobstore.cc
 * @author:	Jacob Xie
 * @date:	2023/12/21 16:58:24 Thursday
 * @brief:
 **/

#include "cxx-demo/include/blobstore.h"
#include "cxx-demo/src/main.rs.h"
#include <algorithm>
#include <functional>
#include <set>
#include <string>
#include <unordered_map>

// Toy implementation of an in-memory blobstore.
//
// In reality the implementation of BlobstoreClient could be a large complex C++ library.
class BlobstoreClient::impl
{
  friend BlobstoreClient;
  using Blob = struct
  {
    std::string data;
    std::set<std::string> tags;
  };
  std::unordered_map<uint64_t, Blob> blobs;
};

BlobstoreClient::BlobstoreClient() : impl(new class BlobstoreClient::impl)
{
}

// BlobstoreClient::BlobstoreClient() {}

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

// Add tag to an existing blob.
void BlobstoreClient::tag(uint64_t blobid, rust::Str tag) const
{
  impl->blobs[blobid].tags.emplace(tag);
}

// Retrieve metadata about a blob.
BlobMetadata BlobstoreClient::metadata(uint64_t blobid) const
{
  BlobMetadata metadata{};
  auto blob = impl->blobs.find(blobid);
  if (blob != impl->blobs.end())
  {
    metadata.size = blob->second.data.size();
    std::for_each(
        blob->second.tags.cbegin(),
        blob->second.tags.cend(), [&](auto& t)
        { metadata.tags.emplace_back(t); }
    );
  }

  return metadata;
}

std::unique_ptr<BlobstoreClient> new_blobstore_client()
{
  return std::unique_ptr<BlobstoreClient>(new BlobstoreClient());
}
