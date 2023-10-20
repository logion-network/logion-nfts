# Assets to prepare 

3 files per piece of Art:
* a json file containing **metadata** (for example `0.json`), publicly available (http ) - used by wallets.
* a **thumbnail** image, (for example `thumb-0.jpeg`) publicly available at the same location - used by wallets.
* an **high-resolution image** (for example `hires-0.jpeg`), to be added to LOC just after items import (CSV), only available via logion restricted delivery.

## Metadata

One json file must be generated and made available via http.

### Prerequisite

#### Values to be determined
* `image`: the url of the thumbnail
* `collectionLocId`: the ID of the LOC

#### `itemId` Calculation
`itemId` is equal to the hash (SHA256) of the following string: `:U64(n)` where n ranges from `0` to `size`-1 (`size` = numbers of NFT's)
For example, `itemId` for `0` is `0xfd8e45608baccf004189a794eee8947ad095dd561e0981fcae90309fac5cf8fe`.
It can be obtained with the following Linux command (Don't forget to add `0x`):

`echo -n ":U64(0)" | sha256sum`

```json 
{
  "name": "ParisDotComm#0",
  "description": "The ParisDotComm NFTs, created by Seneca and certified by Logion",
  "attributes": [
    {
      "trait_type": "Artist",
      "value": "Seneca"
    }
  ],
  "image": "https://raw.githubusercontent.com/logion-network/logion-nfts/example-metadata/metadata/paris-dot-comm/thumb-0.jpg",
  "external_url": "https://certificate.logion.network/public/certificate/${collectionLocId}/${itemId}"
}
```

## CSV

CSV is a convenient way to add many items to a collection LOC in one go.

### Prerequisite

The Smart Contract must be instantiated !

* `TOKEN TYPE` : `astar_shibuya_psp34` | `astar_psp34` 
* `contractAddress` is the address of the contract.

For each token:
* `ID`: `:U64(n)`.
* `FILE NAME`: the hires file name: `hires-n.jpg`.
* `FILE SIZE`: the hires file size.
* `FILE HASH`: the hash (SHA256) of the hires file content. 


```csv
ID,DESCRIPTION,FILE NAME,FILE CONTENT TYPE,FILE SIZE,FILE HASH,RESTRICTED,TOKEN TYPE,TOKEN ID,TOKEN ISSUANCE,TERMS_AND_CONDITIONS TYPE,TERMS_AND_CONDITIONS PARAMETERS
:U64(0),ParisDotComm#0,hires-0.jpg,image/jpeg,135342,0x5048bae5d399fa8421ae0ddc5c39f67696953fa5df514b41d913736c406533c3,Y,astar_shibuya_psp34,"{""contract"":""${contractAddress}"",""id"":{""U64"":0}}",1,logion_classification,"{""transferredRights"": [""PER-PRIV"", ""WW"", ""NOTIME""]}"
:U64(1),ParisDotComm#1,hires-1.jpg,image/jpeg,139651,0xce9f9fe2ac5d989792c229cccf88bf60f6011c9387c1ba9e7dee40364b9ea726,Y,astar_shibuya_psp34,"{""contract"":""${contractAddress}"",""id"":{""U64"":1}}",1,logion_classification,"{""transferredRights"": [""PER-PRIV"", ""WW"", ""NOTIME""]}"
```

#### Hash calculation

The hash content can be calculated with the following Linux command:
(Again, don't forget to add the prefix `0x`)

`sha256sum hires-0.jpg`