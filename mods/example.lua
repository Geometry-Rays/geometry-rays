return {
    enabled = true,

    setup = function()
        print("Example mod loaded!")
        default_gravity:set(0.8)
    end,

    -- This runs every frame
    loop = function()
        -- velocity_y:set(0.5)
    end
}