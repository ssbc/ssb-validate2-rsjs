const validate = require('.');
//const ssbValidate = require("ssb-validate");

/*
const message = ssbValidate.create(
  state.feeds[alice.public],
  alice,
  null,
  exampleObject,
  Date.now()
);
*/

//msg1 = {"hello": "there"}
//msg2 = {"what's": "up"}

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

msg1Json = JSON.stringify(msg1)
msg2Json = JSON.stringify(msg2)

m1 = {
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

m2 = {
    previous: "%/v5mCnV/kmnVtnF3zXtD4tbzoEQo4kRq/0d/bgxP1WI=.sha256",
    author: "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
    sequence: 2,
    timestamp: 1470187292812,
    hash: "sha256",
    content: {
        type: "about",
        about: "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
        image: {
            link: "&MxwsfZoq7X6oqnEX/TWIlAqd6S+jsUA6T1hqZYdl7RM=.sha256",
            size: 642763,
            type: "image/png",
            width: 512,
            height: 512
        }
    },
    signature: "j3C7Us3JDnSUseF4ycRB0dTMs0xC6NAriAFtJWvx2uyz0K4zSj6XL8YA4BVqv+AHgo08+HxXGrpJlZ3ADwNnDw==.sig.ed25519"
}

//console.log(msg1Json)
//console.log(msg2Json)

m1Json = JSON.stringify(m1)
m2Json = JSON.stringify(m2)

//console.log(validate(msg1Json, msg2Json))
console.log(validate(m1Json, m2Json))
