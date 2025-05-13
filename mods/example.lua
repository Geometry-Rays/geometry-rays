return {
    enabled = true,

    setup = function()
        print("Example mod loaded!")
    end,

    -- This runs every frame
    loop = function()
        velocity_y:set(-1.0)
    end
}