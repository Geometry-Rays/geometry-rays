return {
    enabled = true,

    setup = function()
        print("Example mod loaded!")
        default_gravity:set(0.8)
    end,

    -- This runs every frame
    loop = function()
        -- velocity_y:set(0.5)
    end,

    draw = function ()
        if game_state:get() == "Menu" then
            draw_text("Low gravity mod!", 10.0, 590.0, 30, 255, 255, 255, 255)
        end
    end
}