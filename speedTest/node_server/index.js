"use strict";

const restify = require('restify');  
const app = restify.createServer();

//app.use(restify.queryParser());  
//app.use(restify.bodyParser());

app.get('/', (req, res, next) => {  
    res.send('Hello, world!');
});

app.listen(8080, () => {  
    console.log('%s listening at %s', app.name, app.url);
});
