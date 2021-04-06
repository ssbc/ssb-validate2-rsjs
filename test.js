const validate = require('.');

msg1 = {
    previous: null,
    author: "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
    sequence: 1,
    timestamp: 1470186877575,
    hash: "sha256",
    content: {
        type: "about",
        about: "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
        name: "Piet"
    },
    signature: "QJKWui3oyK6r5dH13xHkEVFhfMZDTXfK2tW21nyfheFClSf69yYK77Itj1BGcOimZ16pj9u3tMArLUCGSscqCQ==.sig.ed25519"
}

msg2 = {
    previous: "%/v5mCnV/kmnVtnF3zXtD4tbzoEQo4kRq/0d/bgxP1WI=.sha256",
    author: "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
    sequence: 2,
    timestamp: 1470187292812,
    hash: "sha256",
    content: {
        type: "about",
        about: "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
        image: {
            "link": "&MxwsfZoq7X6oqnEX/TWIlAqd6S+jsUA6T1hqZYdl7RM=.sha256",
            "size": 642763,
            "type": "image/png",
            "width": 512,
            "height": 512
        }
    },
    signature: "j3C7Us3JDnSUseF4ycRB0dTMs0xC6NAriAFtJWvx2uyz0K4zSj6XL8YA4BVqv+AHgo08+HxXGrpJlZ3ADwNnDw==.sig.ed25519"
}

m1 = {"previous":null,"sequence":1,"author":"@Nf/mtdhqYAoht4kQmTD6/QX2A34s8IPnUhX28/OzIXM=.ed25519","timestamp":1438787025000,"hash":"sha256","content":{"type":"post","text":"OLDESTMSG Incididunt fugiat consectetur in sunt et dolor sint elit laborum ut occaecat mollit incididunt irure sunt. Ullamco laboris reprehenderit anim labore est ullamco commodo anim aliqua deserunt voluptate. Amet voluptate esse aliquip incididunt exercitation anim id deserunt. Id adipisicing adipisicing in mollit sit esse qui sunt eu do ut sit amet. Dolor nulla qui sit aliquip occaecat veniam occaecat aliquip nisi nulla Lorem     dolor. Laboris minim in officia quis consectetur sint sit sit sit anim sit. Culpa esse ex sint laborum cupidatat elit. Proident cupidatat exercitation proident nulla.\nAmet qui duis in fugiat nulla ut mollit culpa. Sit        consequat eiusmod labore eiusmod aliqua amet. Consequat voluptate aute ad ex do in minim deserunt dolor. Quis elit exercitation ex cupidatat. Culpa adipisicing incididunt ipsum id dolor aliqua ullamco. Incididunt in cillum    sit id aute culpa excepteur. Commodo fugiat culpa ullamco magna labore exercitation minim fugiat minim labore ut id laborum.\nAdipisicing qui aliquip ullamco nostrud exercitation magna duis in adipisicing. Qui in ad ex nulla  cillum ipsum non ullamco. Nisi sit non consequat ipsum mollit culpa et culpa amet tempor in cillum qui ipsum in. Voluptate sint id sint duis et enim nisi voluptate amet veniam. Tempor reprehenderit exercitation velit ad enim  aute sunt ea eu duis eu.\nNostrud qui labore officia amet est ipsum incididunt do est eu esse. Consequat veniam nisi laboris anim fugiat incididunt occaecat consequat veniam quis aute pariatur do deserunt. Dolor labore veniam est tempor occaecat eu eiusmod minim aute sunt sunt voluptate elit. Occaecat nisi excepteur ut anim incididunt Lorem aliqua esse quis minim amet sit anim ullamco eiusmod. Esse amet reprehenderit non dolore ut pariatur ad elit duis ipsum ex. Sit laboris et pariatur aliqua et elit aliquip sint irure. In quis elit est veniam duis cupidatat est ut fugiat irure sint laborum id. Proident quis cillum quis cupidatat eiusmod veniam anim ex mollit aliqua    aliquip amet occaecat.","channel":"duis","mentions":[{"link":"@Nf/mtdhqYAoht4kQmTD6/QX2A34s8IPnUhX28/OzIXM=.ed25519","name":"ad incididunt"},{"link":"@Nf/mtdhqYAoht4kQmTD6/QX2A34s8IPnUhX28/OzIXM=.ed25519","name":"ex et"}]},   "signature":"N61MAPVR/Nhq65GzDYq1jxZvP1Pxx+kWDeD2DOXvm4A+VHX5vP/4BrTdsFNp9YYIx7IzeswUtZx2dUroHsRfDQ==.sig.ed25519"}

m2 = {"previous":"%t35OMqZyQbKwrRc+Daht9S7SDcJv41oAUEBt/rvFIL0=.sha256","sequence":2,"author":"@Nf/mtdhqYAoht4kQmTD6/QX2A34s8IPnUhX28/OzIXM=.ed25519",   "timestamp":1438787265000,"hash":"sha256","content":{"type":"post","text":"LATESTMSG Magna ipsum velit in qui eu cillum exercitation sit ex. Magna occaecat ut laborum et. Ullamco anim laboris reprehenderit cillum sunt ea non  dolor. Eiusmod culpa id cupidatat reprehenderit sit commodo. Tempor occaecat exercitation aliqua reprehenderit Lorem velit. Irure esse do labore. Pariatur aliquip adipisicing ex est culpa ea excepteur ex aute tempor aliqua.   \nNisi adipisicing qui proident ipsum officia. Ea eiusmod ea in Lorem voluptate mollit irure occaecat velit irure. Cillum nulla ad incididunt consequat est est dolore ad dolore irure qui enim. Nisi id aliqua ipsum non aute    reprehenderit nostrud irure sint sint laborum. Eiusmod elit veniam aute. Proident laboris deserunt elit minim dolor exercitation duis et commodo cillum minim.","mentions":[{"link":"@Nf/mtdhqYAoht4kQmTD6/QX2A34s8IPnUhX28/      OzIXM=.ed25519","name":"veniam incididunt"},{"link":"@Nf/mtdhqYAoht4kQmTD6/QX2A34s8IPnUhX28/OzIXM=.ed25519","name":"anim in"},{"link":"&ZGVzZXJ1bnRleExvcmVtcXVpbnVsbGFhZGN1cGlkYXQ=.sha256","type":"image/jpeg","size":104560}]},"signature":"jzuc2CMWTi6E4Ooot9HfweYM2NOE81wQ+WHEKB4YhAFNFDMXDBcKC9p83eGyHXf79NJVfhQ7qFLvOf+CBtmaCg==.sig.ed25519"}

// validate the first message value from a feed
console.log(validate.validateSingleMsgValue(m1))
// validated

// validate a message value which is not the first in a feed
// console.log(validate.validateSingleMsgValue(m2))
// Error: internal error in Neon module: The first message of a feed must have seq of 1

// validate two successive message values from a feed
console.log(validate.validateMsgValueChain(m1, m2))
// Error: internal error in Neon module: This feed is forked. Last known good message was as seq: 1
// not sure why this isn't working...might be a bug in ssb-validate? (unlikely but yeah)
