using DlclRpc;

namespace DlclNet.Models;

public struct Pixel : IDtoDerivable<PixelDTO>
{
    public Position Position { get; set; }
    public Color Color { get; set; }

    public PixelDTO ToDto()
    {
        var dto = new PixelDTO
        {
            X = Position.X,
            Y = Position.Y,
            R = Color.R,
            G = Color.G,
            B = Color.B
        };
        return dto;
    }
}