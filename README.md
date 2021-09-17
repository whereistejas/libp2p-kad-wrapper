# A client API around `libp2p` for K-DHT.

- Goals
    1. Be able to share files over K-DHT using libp2p between two farmers.
	
- Key ideas
    1. The idea is to not interact with `libp2p` directly. But, instead to wrap them in an interface/API that is suitable to our needs, let's call this wrapping, the `Client` API.
    2. Do not think in terms of `libp2p`, do not structure the code in terms of `libp2p` constructs and concepts. At least, the `Client` API, should not follow that pattern.
    3. There should be a `run` method, which will spawn an `EventLoop` in the background.
    4. The `Client` API will interact with this `EventLoop`.

- Requirements
    <font color="green">Green</font> is important, <font color="yellow">yellow</font> is not so important.
    1. <font color="green">We want to start a libp2p node, when we are in farming mode.</font>
	2. <font color="yellow">We want to start the node on a seperate thread, maybe? </font>    
	3. <font color="yellow">When the farmer starts, it should load the details of the chunks in its plot and announce itself as a `provider` for those chunks.</font>
    4. <font color="green">We want the node to join an overlay network, which means, we basically want it to connect with a few special nodes that are always up and running, called bootnodes.</font>

- API methods
    ### Implementation Details
    Since we have seperated the work into two logical units: the `Client` API and the `EventLoop`. 

    Both of them will:
    1. Be defined using structs
    2. We will define their behaviour using traits
    3. They will have seperate enums that define their events
