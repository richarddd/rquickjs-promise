const error = async () => {
    throw "ERROR 1";
};

const error2 = async () => {
    return Promise.reject("ERROR 2");
};

console.log("hello");

error().catch(console.log);
error2().catch(console.log);
