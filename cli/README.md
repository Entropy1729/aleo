## Aleo Ledger RESTful API Documentation

This document explains the methods of Aleo's Ledger RESTful API.

### Blocks

#### Latest Block

Returns the block from the head of the canonical chain.

#### Response

|       Parameter       |  Type  |                            Description                            |
|:---------------------:|:------:|:-----------------------------------------------------------------:|
|     `block_hash`      | string |                      The hash of the block.                       |
|    `previous_hash`    | string |                  The hash of the previous block.                  |
|       `header`        | object | The block header containing the state of the ledger at the block. |
|    `transactions`     | object |          The list of transactions included in the block.          |
|      `signature`      | string |                        Block signature.                           |

#### Example Request

```
curl --location --request GET 'localhost:4180/testnet3/latest/block'
```

#### Example Response

The structure is as follows:

``` json
{
    "block_hash": "ab104wlhwgmgzc5qgz5k9d0s8ef2vel99ty0qv8ckmdnlmfzzaqmv9s4ld8yk",
    "previous_hash": "ab12rvkruga204qvwlssmmw49u4qh80gd7y697y53j3n35w9z3kngrq5k6vqs",
    "header": {
        "previous_state_root": "2914015866665192428801600606705604399892352785717204915104019770324938411503field",
        "transactions_root": "3912899173042620426668509699847079999728407965567771336753458007787059159551field",
        "metadata": {
            "network": 3,
            "round": 536,
            "height": 536,
            "coinbase_target": 18446744073709551615,
            "proof_target": 18446744073709551615,
            "timestamp": 1660287764
        }
    },
    "transactions": [
        {
            "type": "execute",
            "id": "at1kr6rqn2lpkl6f6mm95ehjvz2aj03sz25g3hn5gsgrrhmaars0sqswgf4f4",
            "execution": {
                "edition": 0,
                "transitions": [
                    {
                        "id": "as197w3m93rlmx8j0vyq7en8ssmu4dd0ppkl82mkfggqs4jmnqwgcrsu2d4uj",
                        "program": "credits.aleo",
                        "function": "transfer",
                        "inputs": [
                            {
                                "type": "record",
                                "id": "966220442543213185414985591311902469854704783278942429449591170074851800114field",
                                "tag": "3256534251619797863903658574435546743392358023461821116881348452901345767783field",
                                "origin": {
                                    "commitment": "2660653412042320408896919914981733219636590022050828196105255200017893183731field"
                                }
                            },
                            {
                                "type": "private",
                                "id": "4679187786844396811101453020460724773902477300170746064794496553361833643827field",
                                "value": "ciphertext1qgqfu7xu4lmczgw7jyx3850qsdd4zzuafv5tdtkhwkwrg3uhmy5xjzry5u4ju23eattlhcugkn35nmfvht6pgk2mh90gfkkjda9rl928qslgyq55"
                            },
                            {
                                "type": "private",
                                "id": "1158514483775923213500257396861528440332593787537368698738728082403425627491field",
                                "value": "ciphertext1qyqdzematgh3nsnl4290jveruvfzhhr86w69t9wsgmcacpma35jlqrgj8uah5"
                            }
                        ],
                        "outputs": [
                            {
                                "type": "record",
                                "id": "739706919968280540192699520081607370428835022675213810572119592364838767113field",
                                "checksum": "3831753729779499843211642926426319979300099889374176383148874983415220380195field",
                                "value": "record1qyqsp6zzlnmfphvc83jspz2y3cl6ccl22uqt8sm0uz0madqdklmyk2qqqyqsqn0l2p9nt42je795k5p6vlrshvhhu40xrs9vh7hm7da08qhv48qjqqqgx8ku3ftcxy9akhu39zwacxuvn8j97qutxzv9y4ts3hwx7ge46rsgp75uz"
                            },
                            {
                                "type": "record",
                                "id": "3556214862666296333146378455137653420778121847667626468069893455881254111055field",
                                "checksum": "1869661040706800157641395496771611654538903104624970741168322152698775328442field",
                                "value": "record1qyqsqls3s58cwjlrth4s3wt2mss27xxl52gtsprl8jzz65tsw7xaqaqwqyqspsnv6rr5tye780dfdncwfctafg7n3xheh5mtrr6xagnppx0fyqqfqqqtxel20eyrrmk9qctxgs57klqa665ekq2ze7nwh2l6uptwyte9qpcacs4v2"
                            }
                        ],
                        "proof": "proof1qqqqzqqqqqqqqqqqz0wugq8lp80sq7rdafe6f94rrqyrkxa9z4u0l9mtl6adzcpjamwn637eswepqevtfk3lr9zyvdgqrrj7ass92dgs6j9qj5ud5h44cqwrgyv0xvahj20fuuhsdulg9dmrwes47s0kyw9mjnthghjusuv0qxsaf7kgmav2xhmjyw73x90uvdxuzwd47mn5g53vn36fnwxv9fz64tlnn9agv2rjw78afvrslfa5nqqpn8w0tksqqa3vf0eur84ks24jljuxjxqkn99vcaxnvst4mgl42q2m0rqm6vdm3w9ksdhe20m40etcpk6rje3vq2s2vvkzujju40qthq5tttzgww3acwsr467ajp3hmf8t0wudsxqa2jlj2m2r8s9g4la2qzqucvsf0d0fkcrhd2cy83ayu5x4jw4ecgd5re8vna7kzcp4d08097kqrjw5z8pzumng4fh34yn4uqp0sn82hcr7c24aqmyr96h9sezqcq7v6c4tm06gqf3hfthykuv3jgydzdry4eatgsaxa7heud50syqrxupwk5wsrw8cen7lma8cyhfc4knctp0yaxjv33a4azhec0svxdeayeheesawpl0s69eaz738u25pu5mp82efpwq9lt40kewyrv97huaftnvelam2l89wzme3kt3ta9gj8vg20uexd55cn6f9rsy2kfrgrv73d887rk82eyt992k9sy8zhktckqnu9avu6tlhy56w0cug3xu89q2av4r5z56n7chmd7fr2354s8d83pc8quyywl4jfc0w7ahqq40cxyde6etc39wwnlerdwmxvymq9j96wk7n9qwgwm5z790p8ycdgrwvvngur2feushnetzlqn3c6jcrkgas8juw8vsrl8g6snyx9azm3gau9csrlaxezh3dmwj79m3zkq9jyanegyklhale4uwvy24wdqyhnj0lt5hdvg9zcff9xvrcfdv0yq2gq86arncw82kvulyvqyjlgvn3axdt0j8tq4sttundngmfkaelpgc73yrddampgqcnp4rw3r4n2cadt9plq54v2qxq9suhwlqu6p3qrwcxq60k0rrt70ztk9qzu89zxwr6m9uvam3hlxvsqr3djlf805s3hppd9jxvgcz4sk9asd54ktl79qhtk4nyl53w2msh68ahl0knysgsyqqqqqqqqqqq0xquh4yf6tyr2z4wn02xzp6hdtvurzujjj2nytrr6z7apuawv7zzflkuk2ynk3h5dp35cyg5t2fgzqdcm74mlz3hhlzajj8ez4l0pe0lx2sm2htauwj2gd9t0nr49h0pqs8jwyjnhaw7n32n68h50t7ucs79dyfzrp4sea5vayzwwnkvh5l53m6jpqcf6h6haq2ljkzhgzry6qgqqqa24wd0",
                        "tpk": "6923257150208423513763781343495628439507562832545255402087541789911211813542group",
                        "tcm": "6459226263121120685587425911637297113604580463299767780543690291959913204372field",
                        "fee": 0
                    }
                ]
            }
        }
    ],
    "signature": "sign12gys2athnsr7508hcrgt5vuctnha5u6duvrzztqmqfp2754ajcppp8atr5xqv2dwta2q43uwrq2ptrn0xyx67nsv5qrhewraleeksqd9tvuq555q76fsdckvtuctln42ll8qsp2klkdl5h88su28n3v0p6yq209va02czx8tergp2dlgzp4axkhkvlreladdzth7sn9ahaxs5um0rrj"
}
```

#### Latest Block Height

Returns the number of blocks in the canonical chain.

#### Response

A `number` representing the height of the latest block.

#### Example Request

```
curl --location --request -GET 'localhost/testnet3/latest/height'
```

#### Example Response

``` json
0
```

#### Get Latest Block Hash

Returns the block hash from the head of the canonical chain.

#### Response

A `string` representing the hash of the latest block.

#### Example Request

```
curl -XGET 'localhost/testnet3/latest/hash'
```

#### Example Response

``` JSON
"ab10tu3lq6ys62mwx0g6x2j25n4cmu26ekcc9erqh3n3x88ahnzycpqcxcckw"
```

### Search

#### Get Block From Block Height

Returns the block for the given block height, if it exists in the
canonical chain.

#### Response

|       Parameter       |  Type  |                            Description                            |
|:---------------------:|:------:|:-----------------------------------------------------------------:|
|     `block_hash`      | string |                      The hash of the block.                       |
|    `previous_hash`    | string |                  The hash of the previous block.                  |
|       `header`        | object | The block header containing the state of the ledger at the block. |
|    `transactions`     | object |          The list of transactions included in the block.          |
|      `signature`      | string |                        Block signature.                           |

#### Example Request

```
curl --location --request -GET 'localhost/testnet3/blocks/0'
```

The height must be an integer from 0 to 4,294,967,295 representing the height
of the block searched.

#### Example Response

```json
{
    "block_hash": "ab1mcw9pf92haasgqxq4tq8cdt4g6xt0rsvhfme4nysrtmf27v44c8s7euv9j",
    "previous_hash": "ab1pq907ec6de2swwyd90x2a3getstpjrn345wtyznewpp0zpul35rs9w5xqd",
    "header": {
        "previous_state_root": "5370899414051520910482317686848043736831616529327521193829133109770203764427field",
        "transactions_root": "7054698832679484518202333000368000540917587553724066259567450200404539707820field",
        "metadata": {
            "network": 3,
            "round": 3,
            "height": 3,
            "coinbase_target": 18446744073709551615,
            "proof_target": 18446744073709551615,
            "timestamp": 1660332946
        }
    },
    "transactions": [
        {
            "type": "execute",
            "id": "at1qe975knf76w9gragl33453mu0cmj3m76m6ga3w8ex2fu56jqdyqsltdk3m",
            "execution": {
                "edition": 0,
                "transitions": [
                    {
                        "id": "as10x6qz2e260khacuv3y6ele52al6fadv6twwhna0d3kadzrpxss8ql8d2w9",
                        "program": "credits.aleo",
                        "function": "transfer",
                        "inputs": [
                            {
                                "type": "record",
                                "id": "6762600561342353042385192864716713615809331791176315098667228277746103074700field",
                                "tag": "8032712657395319136054446756227969350572024320837864697160953224437702940604field",
                                "origin": 
                                {
                                    "commitment": "5810553107362124623419432840758200558890540719323010369383544873422695871383field"
                                }
                            },
                            {
                                "type": "private",
                                "id": "1699503158469445162714386891390856745655776900511822872509886375090528561114field",
                                "value": "ciphertext1qgqp3aaqafxzmfmeyly9mksmsm3kfu4wdqe6ckzsxdrnmpvfv5fu5pkejh5tz7k4fcaum7sdnrerjpzn2evp2qwedckfzfn3ughh8ycrpsw2tanf"
                            },
                            {
                                "type": "private",
                                "id": "828564307994735838187395326040887509732066037514536534541152283165413367011field",
                                "value": "ciphertext1qyqt4mxhy8szeu62wufjnqf3ktsrqwhqrnla8xsm9j38f920x8y6yqc65m3yf"
                            }
                        ],
                        "outputs": [
                            {
                                "type": "record",
                                "id": "3106013856105508916851966776621132948033435260965835137325764410593453738821field",
                                "checksum": "3072903920946395424928813810608808467891254110655716664905449132484852717342field",
                                "value": "record1qyqsp7zl55a0t8tx67eg4nrmz99xdtu3rfyhx2wz20u933qtutzt4qqyqyqsp2qm5vy0hm842c3hp0vnnfs25w7fn9du24kllx84rm0ld3584as9qqqxyfs22m5uftlstry4n40sz4k8n5zrxrqnu0jcrmemajgplfn7cqg0juweq"
                            },
                            {
                                "type": "record",
                                "id": "349983233029330062685502821960005686601264540015795774916525491713522143378field",
                                "checksum": "4765629540898245458094770243675119328097416923399196638211532521268348879338field",
                                "value": "record1qyqsq5s2rea9alqtnd8wrcn4dyj33xg6xcryvc5dl6g4fn98cqsrm2qzqyqsqyjdxsz98ad6pn24qw4s9gep5a4fyzy6rsxncsjkh5d0hjctafsgqqqdsegj6rx6a89y2z37mlvnl3x8a7h27dq9kdxhrds7xj8cnhcyxzql0dl8c"
                            }
                        ],
                        "proof": "proof1qqqqzqqqqqqqqqqqmner40v95zc3sf5cp04zywdwctvaqlldtlynvps6ej6huv8zcarxk4tucl92hx7v87y5ksm40teqqg345nwejnnr9leg9psj84dq5upkz7r6jfa2h0yw4z7rwqjk7vk7gvfz40mg7zwjk2ha7a869j3ksz8kkq45zvljfcxegcy20yudds8zvcdzujgujq9vdfmgazs3awrdwgyjeqeee342q785ume703zf4qqpqn8jfl0zzuah6jt0wwcmkxq49ar8r8klrx7jksm789p65vyc6mkw9gpkz8jd4gy37l3g9y6dnwzgqacf5g5le8tzte70maln4ys895pmt0jxcaplzzjp7r2ehqqu9y02l0xw9cphd8z076pvm7qjmatcsx0z2qpnvz32w77u0stk5jquhrxhyc5pvfl9j3rvzec8gz0w3kk229zn5ckuw9m2rhmwa2ql36xp8qvm6dvs9zr9ut0eaushgzs43q8zadzv4h94ar2u224qxajfjpk3gmmt04yg0rw7gfmqvcclfm8f9cqgr2aykeupuaxjrearvz5d3gw74pudcf2jc8peudlcaggg3uyam2mqvh8d43xups0wxcrnp7vyfuyq4m9vafezjtz0f88xf7mks625ag4n52har9t0fzau7s2ltxssa7wvrs6ezndrr7l5x80k2wzfnvfqr9pzg9687aqelg23fml3mtek6qjgx6d8jrk902d3qh9ar55nnxmy7e4ueqjag2sh6w5qum2ptz27qznems60mxn62w0z2v9nmf5nu48w60jesjtcf6zgxem4y7ztfyp3z78q24e4gxmjzl5hcma877ct7nd4cq7jpxz5qj4h7yzckex3jncgnkvafaud8p6p9jt5uafw6xylwzazpv8mxqpjmfetya8ef8pal5rxwdw38w5m96npe4hn2mgqjwztlsn0kmwhsejtwmetky8el2ytgrj0addfwz8wse7rmhxmd0j5pgvkg0zg0wg69j7avmnwcd9tu37gpweyw37segq92p86ns8ens4t5cr48hc9xp30smnjycjuju2qq5ks9xvk0ezg9zud7sfqkumv5mse59wxx0g4744gmcznjvanvlvnjgqjags6hy69mckvazqsd6pc2lgh9t6ry5aasvh0c9uxuyyxruvr9v8qyqqqqqqqqqqqc0nc6umnrqnlx0g6rxpc632r9mdqykftle7gfmxcn0z2e4rw96syg6x7hyp76pc8g9f4v0g7ujycqqwu0m0fjxtvg44rpjg9vwhmh3yfjrfxfk55mw2pfqgrwxmg09p2p0z2nunk7pfq984dhc863l5fj8kz6lspec4wsqg6w8njvnyeyx75fk0r0403n6lu8weuukjnw4m6aqqqqqn67s4g",
                        "tpk": "4494917511893384371122509268973958427766735367686742879707813315341528817559group",
                        "tcm": "4099251410507696264717500958544815812473090856035015690850577111378300480501field",
                        "fee":0
                    }
                ]
            }
        }
    ],
    "signature": "sign1az5s74u80z0mqjccl49sznkc4csf7egf60sn6540gqxh0uqk0upnhx2w7v3zx572huywr4s7hd46pznvzrusv3a8z4avtylp3jz65qs9w4khrz66fg5fll7w98fs2frfq70f3m9qynpctr68clwqg9g4qf3jy3qj90vker30hanf8zrw0hhdj5pdh2pvlwm2u6vaez5saaxsjpezdur"
}
```

### State Path

TODO

### Records 

#### Records all

Returns all the records in the ledger owned by a user using its view key.

#### Response

|     Parameter      |  Type  |               Description                |
|:------------------:|:------:|:----------------------------------------:|
|   `TODO`  | object |    TODO     |

**Record ID?**

|  Parameter   | Type | Description |
|:--------:|:--------:|:---------:|
|   `owner`  | aleo.group |   Aleo address that owns the record |
|  `gates`   |   u64 | Amount of credits that the record has to spend |
| `_nonce`   | aleo.group | TODO |

#### Example Request

`curl --location --request GET '127.0.0.1:4180/testnet3/records/all' -H 'Content-Type: application/json' -d '"AViewKey1cwUx7j4YYPnwibMhRTQZkKs4HdEV47pFHQJcAkUMXNcF"'`

#### Example Response

```json
{
  "2153026124620299619462017161096869829711340257093989086312824624175512204608field": {
      owner: aleo136wkcln0pfushrsh4229l292ynyvx7pdlc8jjjahuh7kkxjsaugqf6q3x7.private, 
      gates: 1100000000000000u64.private,
      _nonce: 4185076965400754321519878699721977553075640289664060898195995911364718580243group.public
  }
}
```

#### Unspent Records (WIP)

Returns all the unspent records in the ledger owned by a user using its view key.

#### Response 

|     Parameter      |  Type  |               Description                |
|:------------------:|:------:|:----------------------------------------:|
|   `TODO`  | object |    TODO     |

**Record ID?**

|  Parameter   | Type | Description |
|:--------:|:--------:|:---------:|
|   `owner`  | aleo.group |   Aleo address that owns the record |
|  `gates`   |   u64 | Amount of credits that the record has to spend |
| `_nonce`   | aleo.group | TODO |

#### Example Request

`curl --location --request GET '127.0.0.1:4180/testnet3/records/unspent' -H 'Content-Type: application/json' -d '"AViewKey1cwUx7j4YYPnwibMhRTQZkKs4HdEV47pFHQJcAkUMXNcF"'`

#### Example Response

```json
    "2153026124620299619462017161096869829711340257093989086312824624175512204608field": {
        owner: aleo136wkcln0pfushrsh4229l292ynyvx7pdlc8jjjahuh7kkxjsaugqf6q3x7.private, 
        gates: 1100000000000000u64.private,
        _nonce: 4185076965400754321519878699721977553075640289664060898195995911364718580243group.public
    }
```

### Transaction Broadcast

TODO
